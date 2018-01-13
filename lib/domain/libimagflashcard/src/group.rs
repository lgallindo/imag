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

use libimagstore::store::Store;
use libimagstore::store::FileLockEntry;
use libimagstore::storeid::StoreId;
use libimagstore::storeid::StoreIdIterator;

use error::Result;
use session::SessionIterator;

pub trait CardGroup {
    // Based on libimagentrylink

    fn is_cardgroup(&self) -> Result<bool>;
    fn group_name(&self) -> Result<String>;

    fn create_card<'a>(&self, store: &'a Store, question: String, answers: Vec<String>)
        -> Result<FileLockEntry<'a>>;

    fn get_cards<'a>(&self, store: &'a Store) -> Result<Vec<FileLockEntry<'a>>>;

    fn make_session<'a>(&self, store: &'a Store) -> Result<FileLockEntry<'a>>;

    fn all_sessions<'a>(&self, store: &'a Store) -> Result<SessionIterator>;

    // TODO: Some stat-functions for the group
    // like percent learned
    // no of cards
    // no of learned cards
    // etc

}

