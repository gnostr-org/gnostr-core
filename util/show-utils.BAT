@setLocal
@echo off

@rem ::# spell-checker:ignore (CMD) ERRORLEVEL
@rem ::# spell-checker:ignore (utils) cksum coreutils dircolors hashsum mkdir mktemp printenv printf readlink realpath rmdir shuf tsort unexpand
@rem ::# spell-checker:ignore (jq) deps startswith

set "ME=%~0"
set "ME_dir=%~dp0."
set "ME_parent_dir=%~dp0.\.."

@rem refs: <https://forge.rust-lang.org/release/platform-support.html> , <https://docs.rs/platforms/0.2.1/platforms/platform/tier1/index.html>

@rem :: default ("Tier 1" cross-platform) utility list
set "default_utils=base32 base64 basename bech32 cat cksum cli comm cp cut date dircolors dirname echo encrypt_privkey env expand expr factor false fmt fold get_relays git hashsum head join keypair legit link ln ls mkdir mktemp more mv nip11 nl od paste post post_event printenv printf privkey_to_bech32 ptx pubkey_to_bech32 pwd readlink realpath reflog rm rmdir seq shred shuf sleep sort split sum tac tail tee template test tr true truncate tsort tui unexpand uniq wc weeble wobble xq yes"

set "project_dir=%ME_parent_dir%"
cd "%project_dir%"

@:: `jq` available?
set "JQ="
set "ERRORLEVEL="
jq --version 1>NUL 2>&1
if NOT ERRORLEVEL 1 ( set "JQ=jq" )

if NOT DEFINED JQ (
    echo WARN: missing `jq` ^(install with `scoop install jq`^)^; falling back to default ^(only fully cross-platform^) util list 1>&2
    echo %default_utils%
) else (
    cargo metadata %* --format-version 1 | jq -r "[.resolve.nodes[] | { id: .id, deps: [.deps[] | { name:.name, pkg:.pkg }] }] | .[] | select(.id|startswith(\"coreutils\")) | [.deps[] | select((.name|startswith(\"uu_\")) or (.pkg|startswith(\"uu_\")))] | [.[].pkg | match(\"^^\\w+\";\"g\")] | [.[].string | sub(\"^uu_\"; \"\")] | sort | join(\" \")"
    REM cargo metadata %* --format-version 1 | jq -r "[.resolve.nodes[] | { id: .id, deps: [.deps[] | { name:.name, pkg:.pkg }] }] | .[] | select(.id|startswith(\"coreutils\")) | [.deps[] | select((.name|startswith(\"uu_\")) or (.pkg|startswith(\"uu_\")))] | [.[].pkg | match(\"^^\\w+\";\"g\")] | [.[].string] | sort | join(\" \")"
)
