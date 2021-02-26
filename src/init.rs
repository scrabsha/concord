use std::fs;
use std::os::unix::fs as ufs;

use seahorse::{Command, Context};

use crate::utils::{self, AppError, Config};

fn migrate_initial_data(profile_name: &str) -> Result<(), AppError> {
    let discord_config_path = utils::discord_config_dir();
    let storage_path = Config::profile_storage_path(profile_name);

    fs::rename(discord_config_path.as_path(), storage_path.as_path())?;
    ufs::symlink(storage_path, discord_config_path)?;

    Ok(())
}

fn run_inner(initial_profile_name: String) -> Result<(), AppError> {
    let c = Config::create(initial_profile_name)?;
    let profile_name = c.selected_profile;

    migrate_initial_data(profile_name.as_str())?;

    Ok(())
}

fn run(c: &Context) {
    let init_profile_name = c.args[0].as_str();

    if let Err(e) = run_inner(init_profile_name.to_string()) {
        eprintln!("{:#?}", e);
    }
}

pub fn command() -> Command {
    Command::new("init")
        .alias("i")
        .description("initialize concord in the system")
        .usage("concord init [initial_profile_name]")
        .action(run)
}
