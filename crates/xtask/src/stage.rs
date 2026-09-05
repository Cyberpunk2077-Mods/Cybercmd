use std::ffi::OsStr;

use anyhow::Result;
use common::file::download;
use xshell::{cmd, Shell};

use crate::config::Config;

pub const RELEASE_ARGS: [&str; 3] = ["-Z", "build-std", "--release"];

pub const TEST_ARGS: [&str; 2] = ["-Z", "build-std=std"];

pub fn stage<I, II>(config: &Config<'_>, sh: &Shell, build_args: &I) -> Result<()>
where
    I: IntoIterator<Item = II> + Clone,
    II: AsRef<OsStr>,
{
    println!("Start: Staging cybercmd");
    let binary_path = artifact_dir(config, build_args);

    println!("Cleanup staging");
    config.paths.clean_staging()?;

    let cargo = &config.cargo_cmd;

    println!("Building cybercmd");
    {
        let pushed_dir = sh.push_dir(&config.paths.root);
        let build_iter = build_args.clone().into_iter();
        cmd!(sh, "{cargo} build {build_iter...} --package cybercmd").run()?;
        drop(pushed_dir);
    }

    println!("Copying cybercmd.dll to cybercmd.asi");
    sh.copy_file(
        binary_path.join("cybercmd.dll"),
        config.paths.staging_plugins.join("cybercmd.asi"),
    )?;

    println!("Done:  Staging cybercmd");

    Ok(())
}

fn artifact_dir<I, II>(config: &Config<'_>, build_args: &I) -> common::path::PathBuf
where
    I: IntoIterator<Item = II> + Clone,
    II: AsRef<OsStr>,
{
    let args: Vec<_> = build_args
        .clone()
        .into_iter()
        .map(|a| a.as_ref().to_os_string())
        .collect();

    let is_release = args.iter().any(|item| item == "-r" || item == "--release");
    let profile = if is_release { "release" } else { "debug" };

    // Prefer explicit --target output dir when present.
    let mut target_triple = None;
    let mut i = 0;
    while i < args.len() {
        let arg = args[i].to_string_lossy();
        if arg == "--target" {
            if let Some(next) = args.get(i + 1) {
                target_triple = Some(next.to_string_lossy().into_owned());
                break;
            }
        } else if let Some(rest) = arg.strip_prefix("--target=") {
            target_triple = Some(rest.to_owned());
            break;
        }
        i += 1;
    }

    if let Some(triple) = target_triple {
        return config.paths.root.join("target").join(triple).join(profile);
    }

    if is_release {
        config.paths.release.clone()
    } else {
        config.paths.debug.clone()
    }
}

#[allow(clippy::module_name_repetitions)]
pub fn stage_add_standalone(config: &Config<'_>) -> Result<()> {
    let global_ini = config.paths.staging_bin.join("global.ini");
    let version_dll = config.paths.staging_bin.join("version.dll");

    println!("Start: Staging standalone files");
    println!("Downloading global.ini");
    download(config.urls.global_ini, global_ini)?;
    println!("Downloading version.dll");
    download(config.urls.version_dll, version_dll)?;
    println!("Done:  Staging standalone files");

    Ok(())
}
