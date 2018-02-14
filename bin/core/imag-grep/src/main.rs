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

#![deny(
    non_camel_case_types,
    non_snake_case,
    path_statements,
    trivial_numeric_casts,
    unstable_features,
    unused_allocation,
    unused_import_braces,
    unused_imports,
    unused_must_use,
    unused_mut,
    unused_qualifications,
    while_true,
)]

#[macro_use] extern crate log;
extern crate clap;
extern crate regex;

extern crate libimagstore;
extern crate libimagrt;
extern crate libimagerror;

use std::io::Write;

use regex::Regex;

use libimagrt::setup::generate_runtime_setup;
use libimagstore::store::Entry;
use libimagerror::trace::MapErrTrace;
use libimagerror::exit::UnwrapExit;
use libimagerror::io::ToExitCode;

mod ui;

struct Options {
    files_with_matches: bool,
    count: bool,
}

fn main() {
    let rt = generate_runtime_setup("imag-grep",
                                    env!("CARGO_PKG_VERSION"),
                                    "grep through entries text",
                                    ui::build_ui);

    let opts = Options {
        files_with_matches    : rt.cli().is_present("files-with-matches"),
        count                 : rt.cli().is_present("count"),
    };

    let mut count : usize = 0;

    let pattern = rt
        .cli()
        .value_of("pattern")
        .map(Regex::new)
        .unwrap() // ensured by clap
        .unwrap_or_else(|e| {
            error!("Regex building error: {:?}", e);
            ::std::process::exit(1)
        });

    let overall_count = rt
        .store()
        .entries()
        .map_err_trace_exit_unwrap(1)
        .into_get_iter()
        .filter_map(|res| res.map_err_trace_exit_unwrap(1))
        .filter(|entry| pattern.is_match(entry.get_content()))
        .map(|entry| show(&entry, &pattern, &opts, &mut count))
        .count();

    let mut out = ::std::io::stdout();

    if opts.count {
        let _ = writeln!(out, "{}", count).to_exit_code().unwrap_or_exit();
    } else if !opts.files_with_matches {
        let _ = writeln!(out, "Processed {} files, {} matches, {} nonmatches",
                 overall_count,
                 count,
                 overall_count - count)
            .to_exit_code()
            .unwrap_or_exit();
    }
}

fn show(e: &Entry, re: &Regex, opts: &Options, count: &mut usize) {
    let mut out = ::std::io::stdout();

    if opts.files_with_matches {
        let _ = writeln!(out, "{}", e.get_location()).to_exit_code().unwrap_or_exit();
    } else if opts.count {
        *count += 1;
    } else {
        let _ = writeln!(out, "{}:", e.get_location()).to_exit_code().unwrap_or_exit();
        for capture in re.captures_iter(e.get_content()) {
            for mtch in capture.iter() {
                if let Some(m) = mtch {
                    let _ = writeln!(out, " '{}'", m.as_str()).to_exit_code().unwrap_or_exit();
                }
            }
        }

        let _ = writeln!(out, "").to_exit_code().unwrap_or_exit();
    }
}

