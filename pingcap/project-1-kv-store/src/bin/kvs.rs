use clap::{App, AppSettings, Arg, SubCommand};
use std::process::exit;
use kvs::KvStoreDisk;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the value of a string key to a string")
                .arg(Arg::with_name("KEY").help("A string key").required(true))
                .arg(
                    Arg::with_name("VALUE")
                        .help("The string value of the key")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the string value of a given string key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove a given key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .get_matches();
    let store = KvStoreDisk::new();

    match store {
        Ok(mut store) => {
            match matches.subcommand() {
                ("set", Some(args)) => {
                    let key = args.value_of("KEY").expect("KEY argument missing");
                    let value = args.value_of("VALUE").expect("VALUE argument missing");

                    match store.set(key.to_owned(), value.to_owned()) {
                        Ok(_) => println!("key: {}, value: {}", key, value),
                        Err(e) => {
                            eprintln!("Failed to set value: {}", e);
                            exit(1);
                        }
                    }
                    exit(0); // Exit code 0 for success
                }
                ("get", Some(args)) => {
                    let key = args.value_of("KEY").expect("KEY argument missing");
                    match store.get(key.to_owned()) {
                        Ok(Some(value)) => println!("key: {}, value: {:}", key, value),
                        Ok(None) => println!("Key not found"),
                        Err(e) => {
                            eprintln!("Failed to get value: {}", e);
                            exit(1);
                        }
                    }
                    exit(0);
                }
                ("rm", Some(args)) => {
                    let key = args.value_of("KEY").expect("KEY argument missing");
                    match store.remove(key.to_owned()) {
                        Ok(Some(value)) => println!("key: {}, value: {:}", key, value),
                        Ok(None) => println!("Key not found"),
                        Err(e) => {
                            eprintln!("Failed to remove key: {}", e);
                            exit(1);
                        }
                    }
                    exit(0);
                }
                _ => unreachable!(),
            }
        },
        Err(e) => {
            eprintln!("Failed to initialize the key-value store: {}", e);
            exit(1); // Exit with an error code
        }
    }

}