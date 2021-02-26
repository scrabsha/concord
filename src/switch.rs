use std::{fs, os::unix::fs as ufs};

use crate::utils::{self, AppError, Config};

use seahorse::{Command, Context};

fn switch_inner(new_profile: String) -> Result<(), AppError> {
    let mut c = Config::read()?;

    if !c.has_profile(new_profile.as_str()) {
        return Err(AppError::UnknownProfile(new_profile));
    }

    let profile_storage_path = Config::profile_storage_path(new_profile.as_str());
    let discord_config_dir = utils::discord_config_dir();

    fs::remove_file(discord_config_dir.as_path())?;
    ufs::symlink(profile_storage_path, discord_config_dir)?;

    c.selected_profile = new_profile;
    c.write()?;

    Ok(())
}

fn switch(c: &Context) {
    let new_profile = c.args[0].clone();

    if let Err(e) = switch_inner(new_profile) {
        eprintln!("{:#?}", e);
    }
}

pub fn command() -> Command {
    Command::new("switch")
        .alias("s")
        .description("switch to an other profile")
        .usage("concord switch [profile]")
        .action(switch)
}
