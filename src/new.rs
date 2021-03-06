use std::fs;

use std::os::unix::fs as ufs;

use seahorse::{Command, Context};

use crate::utils::{self, AppError, Config};

fn new_inner(new_profile: String) -> Result<(), AppError> {
    let mut c = Config::read()?;
    
    if c.has_profile(new_profile.as_str()) {
        return Err(AppError::DuplicateProfile(new_profile));
    }

    let profile_path = Config::profile_storage_path(new_profile.as_str());
    let discord_config_dir = utils::discord_config_dir();

    fs::create_dir_all(profile_path.as_path())?;

    fs::remove_file(discord_config_dir.as_path())?;
    ufs::symlink(profile_path, discord_config_dir)?;

    c.add_profile(new_profile.clone());
    c.selected_profile = new_profile;
    c.write()?;

    Ok(())
}

fn new(c: &Context) {
    let new_profile = c.args[0].clone();

    if let Err(e) = new_inner(new_profile) {
        eprintln!("{:#?}", e);
    }
}

pub fn command() -> Command {
    Command::new("new")
        .alias("n")
        .description("creates a new profile")
        .usage("concord new [profile]")
        .action(new)
}