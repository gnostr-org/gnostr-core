// This file is part of the gnostr-core package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

// spell-checker:ignore (ToDO) fullname

use clap::{crate_version, Arg, ArgAction, Command};
use uucore::{format_usage, help_about, help_usage};
use uucore::error::UResult;

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
    let _args = args.collect_lossy();
    let name_args =  String::from("");
    let suffix =  String::from("");
    print!("{}", encrypt_privkey(&name_args, &suffix));
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
//#[allow(clippy::uninlined_format_args)]

use gnostr_types::PrivateKey;
use zeroize::Zeroize;
mod rpassword;

// Turn a hex private key into an encrypted private key
fn encrypt_privkey(_fullname: &str, _suffix: &str) -> String {

    let private_key_str = rpassword::prompt_password("Private Key (hex or bech32): ").unwrap();

    let private_key = match PrivateKey::try_from_hex_string(&private_key_str) {
        Ok(pk) => pk,
        Err(_) => match PrivateKey::try_from_bech32_string(&private_key_str) {
            Ok(pk) => pk,
            Err(_) => panic!("Did not recognize private key"),
        },
    };

    let mut log_n = rpassword::prompt_password("Enter the log_n rounds (default 0): ").unwrap();
    if log_n.len() == 0 { log_n = String::from("0"); }
    log_n = log_n.trim().to_string();
    let log_n = log_n.parse::<u8>().unwrap();
    let mut password = rpassword::prompt_password("Password (default empty): ").unwrap();
    let encrypted_private_key = private_key
        .export_encrypted(&password, log_n)
        .expect("Could not export encrypted private key");
    password.zeroize();
    encrypted_private_key.to_string()
}
