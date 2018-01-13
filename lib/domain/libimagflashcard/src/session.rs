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

use libimagstore::storeid::StoreIdIterator;
use libimagstore::store::FileLockEntry;
use libimagstore::store::Store;

use chrono::NaiveDateTime;

use error::Result;
use card::Card;

pub trait Session {
    fn is_session(&self) -> Result<bool>;

    fn start_at(&mut self, ndt: &NaiveDateTime) -> Result<()>;
    fn end_at(&mut self, ndt: &NaiveDateTime)   -> Result<()>;

    fn start(&mut self)                         -> Result<()> {
        let now = ::chrono::offset::Local::now().naive_local();
        self.start_at(&now)
    }

    fn end(&mut self)                           -> Result<()> {
        let now = ::chrono::offset::Local::now().naive_local();
        self.end_at(&now)
    }

    fn started_at(&self) -> Result<Option<NaiveDateTime>>;
    fn ended_at(&self)   -> Result<Option<NaiveDateTime>>;

    fn answer(&mut self, card: &Card, answer: &str) -> Result<bool>;

    /// Get the group this session was created for.
    fn group<'a>(&self, store: &'a Store) -> Result<FileLockEntry<'a>>;
}

pub type SessionIterator = StoreIdIterator;

