//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015-2018 Matthias Beyer <mail@beyermatthias.de> and contributors
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

use std::path::Path;
use std::path::PathBuf;
use std::result::Result as RResult;

use vobject::parse_component;

use libimagstore::store::Store;
use libimagstore::store::FileLockEntry;
use libimagstore::storeid::StoreIdIterator;
use libimagentryref::refstore::RefStore;
use libimagentryref::refstore::UniqueRefPathGenerator;
use libimagentryutil::isa::Is;

use contact::IsContact;
use error::ContactError as CE;
use error::Result;
use util;

struct UniqueContactPathGenerator;
impl UniqueRefPathGenerator for UniqueContactPathGenerator {
    type Error = CE;

    /// The collection the `StoreId` should be created for
    fn collection() -> &'static str {
        "contact"
    }

    /// A function which should generate a unique string for a Path
    fn unique_hash<A: AsRef<Path>>(path: A) -> RResult<String, Self::Error> {
        unimplemented!()
    }

}

pub trait ContactStore<'a> : RefStore<'a> {

    // creating

    fn create_from_path(&'a self, p: &PathBuf) -> Result<FileLockEntry<'a>>;

    /// Create contact ref from buffer
    ///
    /// Needs the `p` argument as we're finally creating a reference by path, the buffer is only for
    /// collecting metadata.
    fn create_from_buf<P: AsRef<Path>>(&'a self, p: P, buf: &String) -> Result<FileLockEntry<'a>>;

    // getting

    fn all_contacts(&'a self) -> Result<StoreIdIterator>;
}

/// The extension for the Store to work with contacts
///
/// The contact functionality is implemented by using the `libimagentryref` library, so basically
/// we only reference vcard files from outside the store.
///
/// Because of this, we do not have an own store collection `/contacts` or something like that, but
/// must stress the `libimagentryref` API for everything.
impl<'a> ContactStore<'a> for Store {

    fn create_from_path(&'a self, p: &PathBuf) -> Result<FileLockEntry<'a>> {
        util::read_to_string(p).and_then(|buf| self.create_from_buf(p, &buf))
    }

    /// Create contact ref from buffer
    fn create_from_buf<P: AsRef<Path>>(&'a self, p: P, buf: &String) -> Result<FileLockEntry<'a>> {
        let component = parse_component(&buf)?;
        debug!("Parsed: {:?}", component);

        RefStore::create_ref::<UniqueContactPathGenerator, P>(self, p)
            .map_err(From::from)
            .and_then(|mut entry| {
                entry.set_isflag::<IsContact>()
                    .map_err(From::from)
                    .map(|_| entry)
            })
    }

    fn all_contacts(&'a self) -> Result<StoreIdIterator> {
        unimplemented!()
    }

}

