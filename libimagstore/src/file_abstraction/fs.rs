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

use std::fs::{File, OpenOptions, create_dir_all, remove_file, copy, rename};
use std::io::{Seek, SeekFrom, Read};
use std::path::{Path, PathBuf};

use error::{MapErrInto, StoreError as SE, StoreErrorKind as SEK};

use super::FileAbstraction;
use super::FileAbstractionInstance;

#[derive(Debug)]
pub enum FSFileAbstractionInstance {
    Absent(PathBuf),
    File(File, PathBuf)
}

impl FileAbstractionInstance for FSFileAbstractionInstance {

    /**
     * Get the content behind this file
     */
    fn get_file_content(&mut self) -> Result<String, SE> {
        debug!("Getting lazy file: {:?}", self);
        let (file, path) = match *self {
            FSFileAbstractionInstance::File(ref mut f, _) => return {
                // We seek to the beginning of the file since we expect each
                // access to the file to be in a different context
                try!(f.seek(SeekFrom::Start(0))
                    .map_err_into(SEK::FileNotSeeked));

                let mut s = String::new();
                f.read_to_string(&mut s)
                    .map_err_into(SEK::IoError)
                    .map(|_| s)
            },
            FSFileAbstractionInstance::Absent(ref p) =>
                (try!(open_file(p).map_err_into(SEK::FileNotFound)), p.clone()),
        };
        *self = FSFileAbstractionInstance::File(file, path);
        if let FSFileAbstractionInstance::File(ref mut f, _) = *self {
            let mut s = String::new();
            f.read_to_string(&mut s)
                .map_err_into(SEK::IoError)
                .map(|_| s)
        } else {
            unreachable!()
        }
    }

    /**
     * Write the content of this file
     */
    fn write_file_content(&mut self, buf: &[u8]) -> Result<(), SE> {
        use std::io::Write;
        let (file, path) = match *self {
            FSFileAbstractionInstance::File(ref mut f, _) => return {
                // We seek to the beginning of the file since we expect each
                // access to the file to be in a different context
                try!(f.seek(SeekFrom::Start(0))
                    .map_err_into(SEK::FileNotCreated));
                f.write_all(buf).map_err_into(SEK::FileNotWritten)
            },
            FSFileAbstractionInstance::Absent(ref p) =>
                (try!(create_file(p).map_err_into(SEK::FileNotCreated)), p.clone()),
        };
        *self = FSFileAbstractionInstance::File(file, path);
        if let FSFileAbstractionInstance::File(ref mut f, _) = *self {
            return f.write_all(buf).map_err_into(SEK::FileNotWritten);
        }
        unreachable!();
    }

}

/// `FSFileAbstraction` state type
///
/// A lazy file is either absent, but a path to it is available, or it is present.
#[derive(Debug)]
pub struct FSFileAbstraction {
}

impl FSFileAbstraction {
    pub fn new() -> FSFileAbstraction {
        FSFileAbstraction { }
    }
}

impl FileAbstraction for FSFileAbstraction {

    fn remove_file(&self, path: &PathBuf) -> Result<(), SE> {
        remove_file(path).map_err_into(SEK::FileNotRemoved)
    }

    fn copy(&self, from: &PathBuf, to: &PathBuf) -> Result<(), SE> {
        copy(from, to).map_err_into(SEK::FileNotCopied).map(|_| ())
    }

    fn rename(&self, from: &PathBuf, to: &PathBuf) -> Result<(), SE> {
        rename(from, to).map_err_into(SEK::FileNotRenamed)
    }

    fn create_dir_all(&self, path: &PathBuf) -> Result<(), SE> {
        create_dir_all(path).map_err_into(SEK::DirNotCreated)
    }

    fn new_instance(&self, p: PathBuf) -> Box<FileAbstractionInstance> {
        Box::new(FSFileAbstractionInstance::Absent(p))
    }
}

fn open_file<A: AsRef<Path>>(p: A) -> ::std::io::Result<File> {
    OpenOptions::new().write(true).read(true).open(p)
}

fn create_file<A: AsRef<Path>>(p: A) -> ::std::io::Result<File> {
    if let Some(parent) = p.as_ref().parent() {
        debug!("Implicitely creating directory: {:?}", parent);
        if let Err(e) = create_dir_all(parent) {
            return Err(e);
        }
    }
    OpenOptions::new().write(true).read(true).create(true).open(p)
}

