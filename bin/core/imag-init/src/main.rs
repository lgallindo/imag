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

extern crate clap;

#[cfg(test)]
extern crate toml;

extern crate libimagerror;

mod ui;

use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::path::Path;
use std::process::Command;

use libimagerror::exit::UnwrapExit;
use libimagerror::io::ToExitCode;

const CONFIGURATION_STR : &'static str = include_str!("../imagrc.toml");

const GITIGNORE_STR : &'static str = r#"
# We ignore the imagrc.toml file by default
#
# That is because we expect the user to put
# this dotfile into his dotfile repository
# and symlink it here.
# If you do not do this, feel free to remove
# this line from the gitignore and add the
# configuration to this git repository.

imagrc.toml
"#;

fn main() {
    let app = ui::build_ui();
    let matches = app.get_matches();
    let mut out = ::std::io::stdout();

    let path = matches
        .value_of("path")
        .map(String::from)
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            ::std::env::var("HOME")
                .map(PathBuf::from)
                .map(|mut p| { p.push(".imag"); p })
                .map(|path| if path.exists() {
                    let _ = writeln!(out, "Path '{:?}' already exists!", path)
                        .to_exit_code()
                        .unwrap_or_exit();
                    let _ = writeln!(out, "Cannot continue.")
                        .to_exit_code()
                        .unwrap_or_exit();
                    ::std::process::exit(1)
                } else {
                    path
                })
                .expect("Failed to retrieve/build path for imag directory.")
        });

    let _ = ::std::fs::create_dir_all(path.clone())
        .expect("Failed to create directory");

    let config_path = {
        let mut config_path = path.clone();
        config_path.push("imagrc.toml");
        config_path
    };

    let _ = OpenOptions::new()
        .write(true)
        .create(true)
        .open(config_path)
        .map(|mut f| {
            let content = if matches.is_present("devel") {
                get_config_devel()
            } else {
                get_config()
            };

            let _ = f.write_all(content.as_bytes())
                .expect("Failed to write complete config to file");
        })
        .expect("Failed to open new configuration file");

    if find_command("git").is_some() && !matches.is_present("nogit") {
        // we initialize a git repository
        let _ = writeln!(out, "Going to initialize a git repository in the imag directory...")
            .to_exit_code()
            .unwrap_or_exit();

        let gitignore_path = {
            let mut gitignore_path = path.clone();
            gitignore_path.push(".gitignore");
            gitignore_path.to_str().map(String::from).expect("Cannot convert path to string")
        };

        let _ = OpenOptions::new()
            .write(true)
            .create(true)
            .open(gitignore_path.clone())
            .map(|mut f| {
                let _ = f.write_all(GITIGNORE_STR.as_bytes())
                    .expect("Failed to write complete gitignore to file");
            })
            .expect("Failed to open new configuration file");

        let path_str = path.to_str().map(String::from).expect("Cannot convert path to string");
        let worktree = format!("--work-tree={}", path_str);
        let gitdir   = format!("--git-dir={}/.git", path_str);

        {
            let output = Command::new("git")
                .args(&[&worktree, &gitdir, "--no-pager", "init"])
                .output()
                .expect("Calling 'git init' failed");

            if output.status.success() {
                let _ = writeln!(out, "{}", String::from_utf8(output.stdout).expect("No UTF-8 output"))
                    .to_exit_code()
                    .unwrap_or_exit();
                let _ = writeln!(out, "'git {} {} --no-pager init' succeeded", worktree, gitdir)
                    .to_exit_code()
                    .unwrap_or_exit();
            } else {
                let _ = writeln!(out, "{}", String::from_utf8(output.stderr).expect("No UTF-8 output"))
                    .to_exit_code()
                    .unwrap_or_exit();
                ::std::process::exit(output.status.code().unwrap_or(1));
            }
        }

        {
            let output = Command::new("git")
                .args(&[&worktree, &gitdir, "--no-pager", "add", &gitignore_path])
                .output()
                .expect("Calling 'git add' failed");
            if output.status.success() {
                let _ = writeln!(out, "{}", String::from_utf8(output.stdout).expect("No UTF-8 output"))
                    .to_exit_code()
                    .unwrap_or_exit();
                let _ = writeln!(out, "'git {} {} --no-pager add {}' succeeded", worktree, gitdir, gitignore_path)
                    .to_exit_code()
                    .unwrap_or_exit();
            } else {
                let _ = writeln!(out, "{}", String::from_utf8(output.stderr).expect("No UTF-8 output"))
                    .to_exit_code()
                    .unwrap_or_exit();
                ::std::process::exit(output.status.code().unwrap_or(1));
            }
        }

        {
            let output = Command::new("git")
                .args(&[&worktree, &gitdir, "--no-pager", "commit", &gitignore_path, "-m", "'Initial import'"])
                .output()
                .expect("Calling 'git commit' failed");
            if output.status.success() {
                let _ = writeln!(out, "{}", String::from_utf8(output.stdout).expect("No UTF-8 output"))
                    .to_exit_code()
                    .unwrap_or_exit();
                let _ = writeln!(out, "'git {} {} --no-pager commit {} -m 'Initial import'' succeeded", worktree, gitdir, gitignore_path)
                    .to_exit_code()
                    .unwrap_or_exit();
            } else {
                let _ = writeln!(out, "{}", String::from_utf8(output.stderr).expect("No UTF-8 output"))
                    .to_exit_code()
                    .unwrap_or_exit();
                ::std::process::exit(output.status.code().unwrap_or(1));
            }
        }

        let _ = writeln!(out, "git stuff finished!")
            .to_exit_code()
            .unwrap_or_exit();
    } else {
        let _ = writeln!("No git repository will be initialized")
            .to_exit_code()
            .unwrap_or_exit();
    }

    let _ = writeln!("Ready. Have fun with imag!")
        .to_exit_code()
        .unwrap_or_exit();
}

fn get_config() -> String {
    get_config_devel()
        .replace(
            r#"level = "debug""#,
            r#"level = "info""#
        )
}

fn get_config_devel() -> String {
    String::from(CONFIGURATION_STR)
}

fn find_command<P: AsRef<Path>>(exe_name: P) -> Option<PathBuf> {
    ::std::env::var_os("PATH")
        .and_then(|paths| {
            ::std::env::split_paths(&paths)
                .filter_map(|dir| {
                    let full_path = dir.join(&exe_name);
                    if full_path.is_file() {
                        Some(full_path)
                    } else {
                        None
                    }
                })
                .next()
        })
}

#[cfg(test)]
mod tests {
    use toml::from_str;
    use toml::Value;
    use super::get_config;
    use super::get_config_devel;

    #[test]
    fn test_config() {
        assert!(from_str::<Value>(&get_config()[..]).is_ok());
        assert!(from_str::<Value>(&get_config_devel()[..]).is_ok());
    }

}
