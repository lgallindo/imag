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

use clap::{Arg, App, ArgGroup, SubCommand};

use libimagentrytag::tag::is_tag;

pub fn build_ui<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
    app.arg(Arg::with_name("id")
                .index(1)
                .takes_value(true)
                .required(true)
                .multiple(false)
                .value_name("ID")
                .help("Entry to use"))

        .arg(Arg::with_name("add-tags")
                .short("a")
                .long("add")
                .takes_value(true)
                .value_name("tags")
                .multiple(true)
                .validator(is_tag)
                .help("Add these tags"))

        .arg(Arg::with_name("remove-tags")
                .short("r")
                .long("remove")
                .takes_value(true)
                .multiple(true)
                .validator(is_tag)
                .value_name("tags")
                .help("Remove these tags"))

       .subcommand(SubCommand::with_name("list")
                   .about("List tags (default)")
                   .version("0.1")
                   .arg(Arg::with_name("json")
                        .long("json")
                        .short("j")
                        .takes_value(false)
                        .required(false)
                        .help("List as JSON"))
                   .arg(Arg::with_name("linewise")
                        .long("linewise")
                        .short("l")
                        .takes_value(false)
                        .required(false)
                        .help("One tag per line"))
                   .arg(Arg::with_name("commasep")
                        .long("comma")
                        .short("c")
                        .takes_value(false)
                        .required(false)
                        .help("Commaseperated (default)"))
                   .arg(Arg::with_name("sep")
                        .long("sep")
                        .short("s")
                        .takes_value(true)
                        .required(false)
                        .help("Separated by string")
                        .value_name("SEP"))

                   .group(ArgGroup::with_name("list-group")
                          .args(&[
                                "json",
                                "linewise",
                                "commasep",
                                "sep",
                          ])
                          .required(true))
                   )

}
