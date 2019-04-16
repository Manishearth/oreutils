use regex::Regex;
use semver::Version;
use std::process::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "oreutils",
    about = "Installation manager for various CLI utilities reimagined in Rust",
    rename_all = "kebab-case"
)]
enum Opt {
    #[structopt(about = "Install the basic utilities: ripgrep, exa, bat, fd")]
    Install,
    #[structopt(
        about = "Upgrade any installed tools. Use `oreutils install` to install missing ones."
    )]
    Upgrade,
    #[structopt(about = "Uninstall all oreutils tools")]
    Uninstall,
}

struct Tool {
    name: &'static str,
    package: &'static str,
    cli: &'static str,
}

const TOOLS: &[Tool] = &[
    Tool {
        name: "ripgrep",
        package: "ripgrep",
        cli: "rg",
    },
    Tool {
        name: "exa",
        package: "exa",
        cli: "exa",
    },
    Tool {
        name: "bat",
        package: "bat",
        cli: "bat",
    },
    Tool {
        name: "fd",
        package: "fd-find",
        cli: "fd",
    },
];

fn main() {
    let opt = Opt::from_args();

    match opt {
        Opt::Install => install(),
        Opt::Upgrade => upgrade(),
        Opt::Uninstall => uninstall(),
    }
}

fn install() {
    for tool in TOOLS {
        let exists = which::which(tool.cli);
        if exists.is_ok() {
            println!(
                "Tool {:?} already installed, use `oreutils upgrade` to upgrade",
                tool.name
            );
            continue;
        }
        cargo_install(tool.package, false);
    }
}

fn upgrade() {
    for tool in TOOLS {
        let res = upgrade_tool(tool);
        match res {
            Ok(vers) => println!("Tool {} updated from version {}", tool.name, vers),
            Err(Error::NotFound) => println!(
                "Tool {} not installed, use `oreutils install` to install",
                tool.name
            ),
            Err(Error::VersionBroken(None)) => {
                println!("`{} --version` didn't produce expected output", tool.cli)
            }
            Err(Error::VersionBroken(Some(v))) => println!(
                "`{} --version` didn't produce expected output: could not parse {}",
                tool.cli, v
            ),
        }
    }
}

enum Error {
    NotFound,
    VersionBroken(Option<String>),
}

fn upgrade_tool(tool: &Tool) -> Result<Version, Error> {
    let output = Command::new(tool.cli)
        .args(&["--version"])
        .output()
        .map_err(|_| Error::NotFound)?;
    let output = String::from_utf8(output.stdout).map_err(|_| Error::VersionBroken(None))?;
    let output = output.lines().next().ok_or(Error::VersionBroken(None))?;
    let re = Regex::new(r"\d+\.\d+\.\d+").unwrap();
    let vers = re
        .find(output)
        .ok_or(Error::VersionBroken(Some(output.into())))?;
    let vers = vers.as_str();
    let vers = Version::parse(vers).map_err(|_| Error::VersionBroken(Some(vers.into())))?;
    cargo_install(tool.package, true);
    Ok(vers)
}

fn uninstall() {
    unimplemented!()
}

fn cargo_install(pkg: &str, force: bool) {
    let mut cmd = Command::new("cargo");
    if force {
        cmd.args(&["install", "-f", pkg]);
    } else {
        cmd.args(&["install", pkg]);
    }
    cmd.env("RUSTFLAGS", "-Ctarget-cpu=native");
    let res = cmd.spawn();
    if res.is_err() {
        eprintln!("Installing {:?} failed", pkg);
    }
}
