// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

// spell-checker:ignore (ToDO) fullname

use clap::{crate_version, Arg, Command};
use std::path::{is_separator, PathBuf};
use uucore::display::Quotable;
use uucore::error::{UResult, UUsageError};
use uucore::{format_usage, help_about, help_usage};

static ABOUT: &str = help_about!("qr.md");

const USAGE: &str = help_usage!("qr.md");

pub mod options {
    pub static DATA: &str = "data";
}


use sha2::{Digest, Sha256};
use std::env;
use std::str;

pub fn sha256_string(data: &str) -> Result<String, String> {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();
    let hex_result = hex::encode(result);
    Ok(hex_result.to_string())
}
pub fn help() {
    use std::process;
    let package_name = env!("CARGO_PKG_NAME");
    print!("{} <data>\n", package_name.replace("_", "-"));
    process::exit(0);
}
pub fn version() {
    use std::process;
    let version = env!("CARGO_PKG_VERSION");
    print!("v{}", version);
    process::exit(0);
}
pub fn default() {
    print!("\ndefault");
    help();
}

use image::Luma;
use qrcode::QrCode;
pub fn render(data: &str) {
    let code = QrCode::new(&data).unwrap();
    let hash = sha256_string(&data).unwrap();
    let image = code.render::<Luma<u8>>().build();
    let location = format!("{}.png", hash);
    image.save(location).unwrap();
    let string = code
        .render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();
    println!("{}", string); //prints blocks to terminal
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    ///cargo test -- --show-output

    #[test]
    fn test_gnostr_tagline() {
        let gnostr_tagline = "gnostr-qr:part of the git+nostr workflow utility";
        render(&gnostr_tagline);
        eprintln!("{:?}", sha256_string(&gnostr_tagline).unwrap());
    }
    #[test]
    fn test_gnostr_github() {
        let gnostr_github = "https://github.com/gnostr-org/gnostr.git";
        render(&gnostr_github);
        eprintln!("{:?}", sha256_string(&gnostr_github).unwrap());
    }
}


#[uucore::main]
pub fn uumain(args: impl uucore::Args) -> UResult<()> {
    let args = args.collect_lossy();

    //print!("LINE:102:uumain");
    //
    // Argument parsing
    //
    let matches = uu_app().try_get_matches_from(args)?;

    let mut name_args = matches
        .get_many::<String>(options::DATA)
        .unwrap_or_default()
        .collect::<Vec<_>>();

    if name_args.is_empty() {
        return Err(UUsageError::new(1, "missing operand".to_string()));
    } else {
        // "simple format"
        match name_args.len() {
            0 => panic!("already checked"),
            1 => String::default(),
            2 => name_args.pop().unwrap().clone(),
            _ => {
                return Err(UUsageError::new(
                    1,
                    format!("extra operand {}", name_args[2].quote(),),
                ));
            }
        }
    };

    //
    // Main Program Processing
    //

    for path in name_args {
    print!("LINE:135:uumain\n");
        print!("{}", qr(path));
    }

    Ok(())
}

pub fn uu_app() -> Command {

    //print!("LINE:144:uumain");
    //use std::env;
    //let _package_name = env!("CARGO_PKG_NAME");
    //let _crate_name = env!("CARGO_CRATE_NAME");
    //let _version = env!("CARGO_PKG_VERSION");
    //let mut args: Vec<String> = env::args().collect();
    //args.remove(0);
    //if args.len() == 1 {
    //    for arg in &args {
    //        if arg == "-h" || arg == "--help" {}
    //        if arg == "-v" || arg == "--version" {}
    //        if arg == "-p" || arg == "--png" {}
    //        render(&arg);
    //        print!("{}\n", &arg);
    //    }
    //    std::process::exit(0);
    //}
    //if args.len() > 1 {
    //    for arg in &args {
    //        if arg == "-h" || arg == "--help" {}
    //        if arg == "-v" || arg == "--version" {}
    //        if arg == "-p" || arg == "--png" {}
    //        render(&arg);
    //        print!("{}\n", &arg);
    //    }
    //    std::process::exit(0);
    //} else {
        //help();
    //}

    //print!("LINE:174:uumain");
    Command::new(uucore::util_name())
        .version(crate_version!())
        .about(ABOUT)
        .override_usage(format_usage(USAGE))
        .infer_long_args(true)
        .arg(
            Arg::new(options::DATA)
                .action(clap::ArgAction::Append)
                .value_hint(clap::ValueHint::AnyPath)
                .hide(true)
                .trailing_var_arg(true),
        )
    //print!("LINE:135:uumain");
}

fn qr(fullname: &str) -> String {
    // Remove all platform-specific path separators from the end.
    let path = fullname.trim_end_matches(is_separator);

    //print!("LINE:194:uumain");
    // If the path contained *only* suffix characters (for example, if
    // `fullname` were "///" and `suffix` were "/"), then `path` would
    // be left with the empty string. In that case, we set `path` to be
    // the original `fullname` to avoid returning the empty path.
    let path = if path.is_empty() { fullname } else { path };

    print!("LINE:135:uumain");
    // Convert to path buffer and get last path component
    let pb = PathBuf::from(path);
    print!("LINE:135:uumain");
    match pb.components().last() {
        Some(c) => {
            let name = c.as_os_str().to_str().unwrap();
                name.to_string()
        }

        None => String::new(),
    }
}
