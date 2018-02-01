//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015, 2016 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

use std::path::PathBuf;

use libimagstore::store::Store;
use libimagstore::store::Entry;
use libimagstore::store::FileLockEntry;
use libimagstore::storeid::IntoStoreId;
use libimagstore::storeid::StoreId;
use libimagstore::storeid::StoreIdIterator;
use libimagentryutil::isa::Is;
use libimagentryutil::isa::IsKindHeaderPathProvider;
use libimagentrylink::internal::InternalLinker;

use toml::Value;
use toml_query::read::TomlValueReadExt;
use toml_query::insert::TomlValueInsertExt;

use error::Result;
use error::FlashcardErrorKind as FCEK;
use card::CardIds;
use card::IsCard;
use session::SessionIds;

provide_kindflag_path!(pub IsCardGroup, "flashcard.is_group");

pub trait CardGroup {
    // Based on libimagentrylink

    fn is_cardgroup(&self) -> Result<bool>;
    fn group_name(&self) -> Result<String>;

    fn create_card<'a>(&mut self, store: &'a Store, question: String, answers: Vec<String>)
        -> Result<FileLockEntry<'a>>;

    fn get_cards<'a>(&self, store: &'a Store) -> Result<CardIds>;

    fn make_session<'a>(&mut self, store: &'a Store) -> Result<FileLockEntry<'a>>;

    fn sessions<'a>(&mut self, store: &'a Store) -> Result<SessionIds>;

    // TODO: Some stat-functions for the group
    // like percent learned
    // no of cards
    // no of learned cards
    // etc

}

impl CardGroup for Entry {
    fn is_cardgroup(&self) -> Result<bool> {
        self.is::<IsCardGroup>().map_err(From::from)
    }

    fn group_name(&self) -> Result<String> {
        match self.get_header().read("flashcard.group.name")? {
            Some(&Value::String(ref s)) => Ok(s.clone()),
            Some(_)                     => Err(FCEK::HeaderTypeError("string")),
            None                        => Err(FCEK::HeaderFieldMissing("flashcard.group.name")),
        }.map_err(Into::into)
    }

    fn create_card<'a>(&mut self, store: &'a Store, question: String, answers: Vec<String>)
        -> Result<FileLockEntry<'a>>
    {
        let name     = format!("{}/{}", self.group_name()?, &question);
        let id       = ::module_path::ModuleEntryPath::new(PathBuf::from(name)).into_storeid()?;
        let mut card = store.create(id)?;

        card.set_isflag::<IsCard>()?;
        {
            let hdr     = card.get_header_mut();
            let answers = answers.into_iter().map(Value::String).collect();

            let _ = hdr.insert("flashcard.card.question", Value::String(question))?;
            let _ = hdr.insert("flashcard.card.answers", Value::Array(answers))?;
        }

        let _ = self.add_internal_link(&mut card)?;
        Ok(card)
    }

    fn get_cards<'a>(&self, store: &'a Store) -> Result<CardIds> {
        let gname = self.group_name()?;
        let iter = store
            .entries()?
            .filter(move |id| id.is_in_collection(&["flashcard", "group", &gname]));

        Ok(CardIds::new(Box::new(iter)))
    }

    fn make_session<'a>(&mut self, store: &'a Store) -> Result<FileLockEntry<'a>> {
        use session::Session;
        use session::IsSession;
        use libimagutil::date::datetime_to_string;
        use module_path::ModuleEntryPath;

        let gname   = self.group_name()?;
        let now     = ::chrono::offset::Local::now().naive_local();
        let id      = format!("session/{}/{}", &gname, datetime_to_string(&now));
        let id      = ModuleEntryPath::new(id).into_storeid()?;
        let mut fle = store.create(id)?;
        let _ = fle.set_isflag::<IsSession>()?;
        let _ = fle.start()?;
        let _ = fle.get_header_mut().insert("flashcard.group.name", Value::String(gname))?;
        let _ = self.add_internal_link(&mut fle)?;
        Ok(fle)
    }

    fn sessions<'a>(&mut self, store: &'a Store) -> Result<SessionIds> {
        use libimagentryutil::isincollection::IsInCollection;
        use filters::filter::Filter;

        struct IsInGroup(Vec<String>);
        impl Filter<StoreId> for IsInGroup {
            fn filter(&self, id: &StoreId) -> bool {
                id.is_in_collection(&self.0)
            }
        }

        let gname = self.group_name()?;
        let f     = IsInGroup(vec!["flashcard".into(), "session".into(), gname]);

        let iter = store.entries()?.filter(move |e| f.filter(&e));
        let iter = StoreIdIterator::new(Box::new(iter));
        Ok(SessionIds::from(iter))
    }

}

pub struct CardGroupIds(StoreIdIterator);

impl From<StoreIdIterator> for CardGroupIds {
    fn from(i: StoreIdIterator) -> Self {
        CardGroupIds(i)
    }
}

impl Iterator for CardGroupIds {
    type Item = StoreId;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.0.next() {
            if next.is_in_collection(&["flashcard", "group"]) {
                return Some(next);
            }
        }

        None
    }
}

