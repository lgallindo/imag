extern crate clap;
use clap::{App, ArgMatches};

struct ConfigBase {
    verbosity : bool,
    debugging : bool,
}

pub struct ModuleConfig {
    base : ConfigBase,
    pub load : bool,
}

impl ModuleConfig {
    pub fn new() -> ModuleConfig {
        ModuleConfig {
            base: ConfigBase {
                verbosity: false,
                debugging: false,
            },
            load: false,
        }
    }
}

pub struct Config {
    base            : ConfigBase,
    pub module_configs  : Vec<ModuleConfig>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            base: ConfigBase {
                verbosity: false,
                debugging: false,
            },
            module_configs: vec![],
        }
    }

    pub fn is_verbose(&self) -> bool {
        self.base.verbosity
    }

    pub fn is_debugging(&self) -> bool {
        self.base.debugging
    }
}

pub fn configure(config : &mut Config) {
    let yaml = load_yaml!("../etc/cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    parse_global_cfg(&matches, &mut config.base);

    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("verbose") {
            println!("Printing verbosely...");
        } else {
            println!("Printing normally...");
        }
    }
}

fn parse_global_cfg(matches : &ArgMatches<>, config : &mut ConfigBase) {
    config.verbosity = matches.is_present("verbose");
    config.debugging = matches.is_present("debugging");
}
