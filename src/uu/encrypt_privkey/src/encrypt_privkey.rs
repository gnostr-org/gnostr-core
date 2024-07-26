// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

// spell-checker:ignore (ToDO) fullname

use clap::{crate_version, Arg, ArgAction, Command};
use std::path::{is_separator, PathBuf};
use uucore::display::Quotable;
use uucore::error::{UResult, UUsageError};
use uucore::line_ending::LineEnding;
use uucore::{format_usage, help_about, help_usage};

static ABOUT: &str = help_about!("encrypt_privkey.md");

const USAGE: &str = help_usage!("encrypt_privkey.md");

pub mod options {
    pub static MULTIPLE: &str = "multiple";
    pub static NAME: &str = "name";
    pub static SUFFIX: &str = "suffix";
    pub static ZERO: &str = "zero";
}

#[uucore::main]
pub fn uumain(args: impl uucore::Args) -> UResult<()> {
    let args = args.collect_lossy();

    //
    // Argument parsing
    //
    let matches = uu_app().try_get_matches_from(args)?;

    let line_ending = LineEnding::from_zero_flag(matches.get_flag(options::ZERO));

    //let mut name_args = matches
      //  .get_one::<String>(options::NAME)
        //.unwrap();
        //.collect::<Vec<_>>();
    let mut name_args: std::string::String = matches
        .get_one::<String>(options::NAME).expect("REASON")
        .to_string();
     if name_args.is_empty() {
         //return Err(UUsageError::new(1, "missing operand".to_string()));
     }
    let multiple_paths =
        matches.get_one::<String>(options::SUFFIX).is_some() || matches.get_flag(options::MULTIPLE);
    let suffix = if multiple_paths {
        matches
            .get_one::<String>(options::SUFFIX)
            .cloned()
            .unwrap_or_default()
     } else {
         // "simple format"
        match name_args.len() {
            //0 => panic!("already checked"),
            //1 => String::default(),
            1 => name_args.pop().unwrap().to_string(),
            _ => {
                return Err(UUsageError::new(
                    1,
                    format!("extra operand {}", name_args.quote(),),
                ));
            }
        }
    };

    //
    // Main Program Processing
    //

    //for path in name_args {
        print!("{}{}", encrypt_privkey(&name_args, &suffix), line_ending);
   // }

    Ok(())
}

pub fn uu_app() -> Command {
    Command::new(uucore::util_name())
        .version(crate_version!())
        .about(ABOUT)
        .override_usage(format_usage(USAGE))
        .infer_long_args(true)
        .arg(
            Arg::new(options::MULTIPLE)
                .short('a')
                .long(options::MULTIPLE)
                .help("support multiple arguments and treat each as a NAME")
                .action(ArgAction::SetTrue)
                .overrides_with(options::MULTIPLE),
        )
        .arg(
            Arg::new(options::NAME)
                .action(clap::ArgAction::Append)
                .value_hint(clap::ValueHint::AnyPath)
                .hide(true)
                .trailing_var_arg(true),
        )
        .arg(
            Arg::new(options::SUFFIX)
                .short('s')
                .long(options::SUFFIX)
                .value_name("SUFFIX")
                .help("remove a trailing SUFFIX; implies -a")
                .overrides_with(options::SUFFIX),
        )
        .arg(
            Arg::new(options::ZERO)
                .short('z')
                .long(options::ZERO)
                .help("end each output line with NUL, not newline")
                .action(ArgAction::SetTrue)
                .overrides_with(options::ZERO),
        )
}




// TEMPORARILY
#[allow(clippy::uninlined_format_args)]

use gnostr_types::PrivateKey;
use zeroize::Zeroize;

// Turn a hex private key into an encrypted private key
fn encrypt_privkey(fullname: &str, suffix: &str) -> String {
    if cfg!(debug_assertions) {
        println!("WARNING: This takes a long time in debug mode.");
    }
    // Remove all platform-specific path separators from the end.
    let path = fullname.trim_end_matches(is_separator);

    // If the path contained *only* suffix characters (for example, if
    // `fullname` were "///" and `suffix` were "/"), then `path` would
    // be left with the empty string. In that case, we set `path` to be
    // the original `fullname` to avoid returning the empty path.
    let path = if path.is_empty() { fullname } else { path };

    // Convert to path buffer and get last path component
    let pb = PathBuf::from(path);
    match pb.components().last() {
        Some(c) => {
            let name = c.as_os_str().to_str().unwrap();
            if name == suffix {
                //name.to_string()
                return name.to_string();
            } else {
                //name.strip_suffix(suffix).unwrap_or(name).to_string()
                return name.strip_suffix(suffix).unwrap_or(name).to_string();
            }
        }

        None => String::new(),
    };

    let private_key_str = rpassword::prompt_password("Private Key (hex or bech32): ").unwrap();

    let private_key = match PrivateKey::try_from_hex_string(&private_key_str) {
        Ok(pk) => pk,
        Err(_) => match PrivateKey::try_from_bech32_string(&private_key_str) {
            Ok(pk) => pk,
            Err(_) => panic!("Did not recognize private key"),
        },
    };

    println!("Enter the logN rounds (a power of 2, e.g. 20): ");
    let mut log_n = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut log_n).unwrap();
    log_n = log_n.trim().to_string();
    let log_n = log_n.parse::<u8>().unwrap();

    let mut password = rpassword::prompt_password("Password: ").unwrap();

    let encrypted_private_key = private_key
        .export_encrypted(&password, log_n)
        .expect("Could not export encrypted private key");
    print!("{}", encrypted_private_key);
    password.zeroize();
    encrypted_private_key.to_string()
}
