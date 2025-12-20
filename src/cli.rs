use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "docsort", version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Clone)]
pub enum Command {
    Start,
    #[command(about = "Manage the autostart behavior")]
    Autostart {
        #[arg(value_enum)]
        action: AutostartAction,
    },
}

#[derive(clap::ValueEnum, Clone)]
pub enum AutostartAction {
    Enable,
    Disable,
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }

    pub fn command(&self) -> Command {
        self.command.clone().unwrap_or(Command::Start)
    }
}
