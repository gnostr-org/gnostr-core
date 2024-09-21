// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

// spell-checker:ignore manpages mangen

use clap::{Arg, Command};
use clap_complete::Shell;
use std::cmp;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process;
use uucore::display::Quotable;

const VERSION: &str = env!("CARGO_PKG_VERSION");

include!(concat!(env!("OUT_DIR"), "/uutils_map.rs"));

fn usage<T>(utils: &UtilityMap<T>, name: &str) {
    println!("{name} {VERSION} (multi-call binary)\n");
    println!("Usage: {name} [function [arguments...]]\n");
    println!("Currently defined functions:\n");
    #[allow(clippy::map_clone)]
    let mut utils: Vec<&str> = utils.keys().map(|&s| s).collect();
    utils.sort_unstable();
    let display_list = utils.join(", ");
    let width = cmp::min(textwrap::termwidth(), 100) - 4 * 2; // (opinion/heuristic) max 100 chars wide with 4 character side indentions
    println!(
        "{}",
        textwrap::indent(&textwrap::fill(&display_list, width), "    ")
    );
}

/// # Panics
/// Panics if the binary path cannot be determined
fn binary_path(args: &mut impl Iterator<Item = OsString>) -> PathBuf {
    match args.next() {
        Some(ref s) if !s.is_empty() => PathBuf::from(s),
        _ => std::env::current_exe().unwrap(),
    }
}

fn name(binary_path: &Path) -> Option<&str> {
    binary_path.file_stem()?.to_str()
}

fn get_arg_count<'a>(mut arg_count: u32) -> u32 {
    let args = std::env::args()
        .skip(1) // skip program name
        .peekable(); // allow looking forward one

    for arg in args {
        arg_count += 1;
        #[cfg(debug_assertions)]
        print!("{}={} ", arg, arg_count);
    }
    #[cfg(debug_assertions)]
    println!("");
    arg_count
}
#[allow(clippy::cognitive_complexity)]
fn main() {
    let mut arg_count: u32 = 0;
    let mut tag_count: u32 = 0;
    let mut e_count: u32 = 0;
    let mut p_count: u32 = 0;
    let mut t_count: u32 = 0;

    let mut content: String = String::new();
    let mut dm: String = String::new();
    let mut envelope: String = String::new();
    let mut kind: String = String::new();
    let mut created_at: String = String::new();
    let mut sec: String = String::new();
    let mut pow: String = String::new();
    let mut mine_pubkey: String = String::new();
    let mut tag: Vec<String> = Vec::new();
    let mut hash: String = String::new();
    let mut e: String = String::new();
    let mut p: String = String::new();
    let mut t: Vec<String> = Vec::new();
    // RELAY=wss://nos.lol PORT=6102 SECRET=0000000000000000000000000000000000000000000000000000000000000001 gnostr-core
    let default_secret: String =
        String::from("0000000000000000000000000000000000000000000000000000000000000001");
    let relay_key: String = String::new();
    let default_relay: String = String::from("wss://e.nos.lol");
    let port_key = String::new();
    let default_port = 6102;

    let args = std::env::args()
        .skip(1) // skip program name
        .peekable(); // allow looking forward one
    #[cfg(debug_assertions)]
    println!("args={:?}", args);

    let mut arg_count = get_arg_count(0);
    #[cfg(debug_assertions)]
    println!("get_arg_count(0)={}", arg_count);
    for arg in args {
        arg_count -= 1;
        #[cfg(debug_assertions)]
        println!("for {} in args={}", arg, arg_count);
    }
    let mut args = std::env::args()
        .skip(1) // skip program name
        .peekable(); // allow looking forward one

    match args.peek().map(|x| x.as_ref()) {
        Some("--content") => {
            #[cfg(debug_assertions)]
            println!("{:?}", args);
            //args.next(); // Skip the flag
            content = args.next().unwrap();
            #[cfg(debug_assertions)]
            println!("82:content={}", content);
        }
        Some("--dm") => {
            #[cfg(debug_assertions)]
            println!("{:?}", args);
            //args.next(); // Skip the flag
            dm = args.next().unwrap();
            #[cfg(debug_assertions)]
            println!("82:content={}", content);
        }
        Some("--envelope") => {
            #[cfg(debug_assertions)]
            println!("{:?}", args);
            //args.next(); // Skip the flag
            envelope.push_str(&args.next().unwrap());
            #[cfg(debug_assertions)]
            println!("87:tag={:?}", tag);
        }
        Some("--kind") => {
            #[cfg(debug_assertions)]
            println!("{:?}", args);
            //args.next(); // Skip the flag
            t.push(args.next().unwrap());
            #[cfg(debug_assertions)]
            println!("87:tag={:?}", tag);
        }
        Some("--created-at") => {
            #[cfg(debug_assertions)]
            println!("{:?}", args);
            //args.next(); // Skip the flag
            t.push(args.next().unwrap());
            #[cfg(debug_assertions)]
            println!("103:nip={:?}", tag);
        }
        Some("--sec") => {
            #[cfg(debug_assertions)]
            println!("{:?}", args);
            //args.next(); // Skip the flag
            t.push(args.next().unwrap());
            #[cfg(debug_assertions)]
            println!("103:nip={:?}", tag);
        }
        Some("--pow") => {
            #[cfg(debug_assertions)]
            println!("{:?}", args);
            //args.next(); // Skip the flag
            t.push(args.next().unwrap());
            #[cfg(debug_assertions)]
            println!("103:nip={:?}", tag);
        }
        Some("--mine-pubkey") => {
            #[cfg(debug_assertions)]
            println!("{:?}", args);
            //args.next(); // Skip the flag
            mine_pubkey = args.next().unwrap();
            #[cfg(debug_assertions)]
            println!("125:args.peek()={:?}", args.peek());
            #[cfg(debug_assertions)]
            println!("126:nip={:?}", mine_pubkey);
        }
        Some("--tag") => {
            #[cfg(debug_assertions)]
            println!("{:?}", args);
            args.next(); // Skip the flag
            tag.push(args.next().unwrap());
            #[cfg(debug_assertions)]
            println!("131:args.peek()={:?}", args.peek());
            #[cfg(debug_assertions)]
            println!("132:secret={:?}", tag);
            tag_count += 1;
        }
        Some("--hash") => {
            #[cfg(debug_assertions)]
            println!("{:?}", args);
            args.next(); // Skip the flag
            hash = args.next().unwrap();
            #[cfg(debug_assertions)]
            println!("138:args.peek()={:?}", args.peek());
            #[cfg(debug_assertions)]
            println!("139:hash={}", hash);
        }
        Some("-e") => {
            #[cfg(debug_assertions)]
            println!("{:?}", args);
            args.next(); // Skip the flag
            e.push_str(&args.next().unwrap());
            #[cfg(debug_assertions)]
            println!("145:args.peek()={:?}", args.peek());
            #[cfg(debug_assertions)]
            println!("146:secret={}", e);
            e_count += 1;
        }
        Some("-p") => {
            #[cfg(debug_assertions)]
            println!("{:?}", args);
            args.next(); // Skip the flag
            p.push_str(&args.next().unwrap());
            #[cfg(debug_assertions)]
            println!("152:args.peek()={:?}", args.peek());
            #[cfg(debug_assertions)]
            println!("153:secret={}", p);
            p_count += 1;
        }
        Some("-t") => {
            #[cfg(debug_assertions)]
            println!("{:?}", args);
            args.next(); // Skip the flag
            t.push(args.next().unwrap());
            #[cfg(debug_assertions)]
            println!("159:args.peek()={:?}", args.peek());
            #[cfg(debug_assertions)]
            println!("160:secret={:?}", t);
            t_count += 1;
        }
        _ => {
            #[cfg(debug_assertions)]
            println!("handle no option");
        }
    } //end match
    #[cfg(debug_assertions)]
    println!("198:tag={:?}", tag);

    for tags in tag {
        #[cfg(debug_assertions)]
        print!("201:t={}", tags);
    }
    for ts in t {
        #[cfg(debug_assertions)]
        print!("204:t={}", ts);
    }
    //}

    //TODO:intercept for gnostr args
    //gnostr --sec <sha256> -t <string> --tag <string> <string> --content <string> --pow <int> --dm <sha256> etc...

    //SECRET=<secret_key> gnostr ...
    if sec.is_empty() {
        sec = match std::env::var("SECRET") {
            Ok(val) => val,
            Err(_) => {
                #[cfg(debug_assertions)]
                println!("default_secret_key={}", default_secret);
                default_secret
            }
        };
    }
    //SECRET=<secret_key> RELAY=wss://<url> gnostr ...
    let relay = match std::env::var("RELAY") {
        Ok(val) => val,
        Err(_) => {
            #[cfg(debug_assertions)]
            println!("relay_key={}", relay_key);
            default_relay
        }
    };

    let port = match std::env::var(port_key) {
        Ok(val) => match val.parse::<u16>() {
            Ok(port) => port,
            Err(_) => {
                println!("default port {} will be used.", default_port);
                default_port
            }
        },
        Err(_) => {
            //println!(
            //  "\"{}\" is not defined in environment variables. default port will be used.",
            //port_key
            //);
            default_port
        }
    };
    #[cfg(debug_assertions)]
    println!("147:sec={}", sec);
    #[cfg(debug_assertions)]
    println!("148:relay={}", relay);
    #[cfg(debug_assertions)]
    println!("149:port={}", port);

    //std::process::exit(0);
    let args = std::env::args();
    for arg in args {
        #[cfg(debug_assertions)]
        println!("224:arg={:?}", arg);
        if arg == "--sec" {
            println!("arg=--sec:\ndo something\n{:?}", arg);
        }
        if arg_count > 1 {
            arg_count -= 1;
        }
    }
    #[cfg(debug_assertions)]
    print!("arg_count={}", arg_count);
    #[cfg(debug_assertions)]
    print!("tag_count={}", tag_count);
    #[cfg(debug_assertions)]
    print!("e_count={}", e_count);
    #[cfg(debug_assertions)]
    print!("p_count={}", p_count);
    #[cfg(debug_assertions)]
    print!("t_count={}", t_count);
    #[allow(unreachable_code)]
    //EXITstd::process::exit(0);
    #[allow(unreachable_code)]
    uucore::panic::mute_sigpipe_panic();

    let utils = util_map();
    let mut args = uucore::args_os();

    let binary = binary_path(&mut args);
    let binary_as_util = name(&binary).unwrap_or_else(|| {
        usage(&utils, "<unknown binary name>");
        process::exit(0);
    });

    // binary name equals util name?
    if let Some(&(uumain, _)) = utils.get(binary_as_util) {
        process::exit(uumain((vec![binary.into()].into_iter()).chain(args)));
    }

    // binary name equals prefixed util name?
    // * prefix/stem may be any string ending in a non-alphanumeric character
    let util_name = if let Some(util) = utils.keys().find(|util| {
        binary_as_util.ends_with(*util)
            && !binary_as_util[..binary_as_util.len() - (*util).len()]
                .ends_with(char::is_alphanumeric)
    }) {
        // prefixed util => replace 0th (aka, executable name) argument
        Some(OsString::from(*util))
    } else {
        // unmatched binary name => regard as multi-binary container and advance argument list
        uucore::set_utility_is_second_arg();
        args.next()
    };

    // 0th argument equals util name?
    if let Some(util_os) = util_name {
        fn not_found(util: &OsStr) -> ! {
            println!("{}: function/utility not found", util.maybe_quote());
            process::exit(1);
        }

        let Some(util) = util_os.to_str() else {
            not_found(&util_os)
        };

        match util {
            "completion" => gen_completions(args, &utils),
            "manpage" => gen_manpage(args, &utils),
            "--list" => {
                let mut utils: Vec<_> = utils.keys().collect();
                utils.sort();
                for util in utils {
                    println!("{util}");
                }
                process::exit(0);
            }
            // Not a special command: fallthrough to calling a util
            _ => {}
        };

        match utils.get(util) {
            Some(&(uumain, _)) => {
                process::exit(uumain((vec![util_os].into_iter()).chain(args)));
            }
            None => {
                if util == "--help" || util == "-h" {
                    // see if they want help on a specific util
                    if let Some(util_os) = args.next() {
                        let Some(util) = util_os.to_str() else {
                            not_found(&util_os)
                        };

                        match utils.get(util) {
                            Some(&(uumain, _)) => {
                                let code = uumain(
                                    (vec![util_os, OsString::from("--help")].into_iter())
                                        .chain(args),
                                );
                                io::stdout().flush().expect("could not flush stdout");
                                process::exit(code);
                            }
                            None => not_found(&util_os),
                        }
                    }
                    usage(&utils, binary_as_util);
                    process::exit(0);
                } else {
                    not_found(&util_os);
                }
            }
        }
    } else {
        // no arguments provided
        usage(&utils, binary_as_util);
        process::exit(0);
    }
}

/// Prints completions for the utility in the first parameter for the shell in the second parameter to stdout
/// # Panics
/// Panics if the utility map is empty
fn gen_completions<T: uucore::Args>(
    args: impl Iterator<Item = OsString>,
    util_map: &UtilityMap<T>,
) -> ! {
    let all_utilities: Vec<_> = std::iter::once("gnostr-core")
        .chain(util_map.keys().copied())
        .collect();

    let matches = Command::new("completion")
        .about("Prints completions to stdout")
        .arg(
            Arg::new("utility")
                .value_parser(clap::builder::PossibleValuesParser::new(all_utilities))
                .required(true),
        )
        .arg(
            Arg::new("shell")
                .value_parser(clap::builder::EnumValueParser::<Shell>::new())
                .required(true),
        )
        .get_matches_from(std::iter::once(OsString::from("completion")).chain(args));

    let utility = matches.get_one::<String>("utility").unwrap();
    let shell = *matches.get_one::<Shell>("shell").unwrap();

    let mut command = if utility == "gnostr-core" {
        gen_coreutils_app(util_map)
    } else {
        util_map.get(utility).unwrap().1()
    };
    let bin_name = std::env::var("PROG_PREFIX").unwrap_or_default() + utility;

    clap_complete::generate(shell, &mut command, bin_name, &mut io::stdout());
    io::stdout().flush().unwrap();
    process::exit(0);
}

/// Generate the manpage for the utility in the first parameter
/// # Panics
/// Panics if the utility map is empty
fn gen_manpage<T: uucore::Args>(
    args: impl Iterator<Item = OsString>,
    util_map: &UtilityMap<T>,
) -> ! {
    let all_utilities: Vec<_> = std::iter::once("gnostr-core")
        .chain(util_map.keys().copied())
        .collect();

    let matches = Command::new("manpage")
        .about("Prints manpage to stdout")
        .arg(
            Arg::new("utility")
                .value_parser(clap::builder::PossibleValuesParser::new(all_utilities))
                .required(true),
        )
        .get_matches_from(std::iter::once(OsString::from("manpage")).chain(args));

    let utility = matches.get_one::<String>("utility").unwrap();
    print!("{}", utility);

    let command = if utility == "gnostr-core" {
        gen_coreutils_app(util_map)
    } else {
        util_map.get(utility).unwrap().1()
    };

    let man = clap_mangen::Man::new(command);
    man.render(&mut io::stdout())
        .expect("Man page generation failed");
    io::stdout().flush().unwrap();
    process::exit(0);
}

/// # Panics
/// Panics if the utility map is empty
fn gen_coreutils_app<T: uucore::Args>(util_map: &UtilityMap<T>) -> Command {
    let mut command = Command::new("gnostr-core");
    for (name, (_, sub_app)) in util_map {
        // Recreate a small subcommand with only the relevant info
        // (name & short description)
        let about = sub_app()
            .get_about()
            .expect("Could not get the 'about'")
            .to_string();
        let sub_app = Command::new(name).about(about);
        command = command.subcommand(sub_app);
    }
    command
}
