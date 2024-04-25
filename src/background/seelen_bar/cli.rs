use clap::Command;

use crate::{error_handler::Result, get_subcommands};

use super::FancyToolbar;

get_subcommands![
    /** Open Dev Tools (only works if the app is running in dev mode) */
    Debug,
];

impl FancyToolbar {
    pub const CLI_IDENTIFIER: &'static str = "toolbar";

    pub fn get_cli() -> Command {
        Command::new(Self::CLI_IDENTIFIER)
            .about("Seelen's Fancy Toolbar")
            .visible_alias("ftb")
            .subcommands(SubCommand::commands())
    }

    pub fn process(&mut self, matches: &clap::ArgMatches) -> Result<()> {
        let subcommand = SubCommand::try_from(matches)?;
        log::trace!("Processing {:?}", subcommand);
        match subcommand {
            SubCommand::Debug => {
                self.window.open_devtools();
            }
        };
        Ok(())
    }
}
