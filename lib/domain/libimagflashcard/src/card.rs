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

use libimagstore::storeid::StoreId;
use libimagstore::storeid::StoreIdIterator;
use libimagentryutil::isa::Is;
use libimagentryutil::isa::IsKindHeaderPathProvider;

provide_kindflag_path!(pub IsCard, "flashcard.is_card");

use error::Result;

pub trait Card {

    fn is_card(&self)    -> Result<bool>;
    fn group_name(&self) -> Result<String>;
    fn question(&self)   -> Result<String>;
    fn answers(&self)    -> Result<Vec<String>>;

}

pub struct CardIds(Box<Iterator<Item = StoreId>>);

impl CardIds {
    pub fn new(i: Box<Iterator<Item = StoreId>>) -> Self {
        CardIds(i)
    }
}

impl Iterator for CardIds {
    type Item = StoreId;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.0.next() {
            if next.is_in_collection(&["flashcard", "card"]) {
                return Some(next);
            }
        }

        None
    }
}

