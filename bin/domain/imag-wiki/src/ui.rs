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

use clap::{Arg, App, SubCommand};

use libimagutil::cli_validators::*;

pub fn build_ui<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
    app
        .subcommand(SubCommand::with_name("add")
                   .about("Add wiki entry")
                   .version("0.1")
                   .arg(Arg::with_name("category")
                        .long("category")
                        .short("c")
                        .takes_value(true)
                        .required(false)
                        .multiple(false)
                        .value_name("NAME")
                        .help("Add the entry under this category."))
                   .arg(Arg::with_name("tags")
                        .long("tags")
                        .short("t")
                        .takes_value(true)
                        .required(false)
                        .multiple(true)
                        .value_name("TAG")
                        .value_names("TAGS")
                        .help("Add these tags to the entry"))
                   .arg(Arg::with_name("name")
                        .index(1)
                        .takes_value(true)
                        .required(true)
                        .multiple(false)
                        .value_name("NAME")
                        .help("Add the entry under this name. The name must be unique, namespaces ("foo/bar") are allowed."))
                   )

        .subcommand(SubCommand::with_name("list")
                   .about("List wiki entries.")
                   .version("0.1")
                   .arg(Arg::with_name("category")
                        .long("categories")
                        .short("c")
                        .takes_value(true)
                        .required(false)
                        .multiple(false)
                        .value_name("CATEGORY")
                        .value_names("CATEGORIES")
                        .help("List only these categories. Categories can be specified as 'supercategory.subcategory'."))
                   .arg(Arg::with_name("tags")
                        .long("tags")
                        .short("t")
                        .takes_value(true)
                        .required(false)
                        .multiple(true)
                        .value_name("TAG")
                        .value_names("TAGS")
                        .help("List only entries with these tags."))

                   .arg(Arg::with_name("grep")
                        .long("grep")
                        .short("g")
                        .takes_value(true)
                        .required(false)
                        .multiple(true)
                        .value_name("PATTERN")
                        .value_names("PATTERNS")
                        .help("List only entries where the content matches the pattern."))

                   .arg(Arg::with_name("no-tree")
                        .long("no-tree")
                        .short("T")
                        .takes_value(false)
                        .required(false)
                        .multiple(false)
                        .help("Do not list entries in human-readable form (as a Tree), but one entry per line (store id). Useful for scripting."))
                   )

        .subcommand(SubCommand::with_name("find")
                   .about("Find entries by grepping through their content.")
                   .version("0.1")
                   .arg(Arg::with_name("grep")
                        .index(1)
                        .takes_value(true)
                        .required(true)
                        .multiple(false)
                        .value_name("PATTERN")
                        .help("Grep pattern."))
                   )
}
