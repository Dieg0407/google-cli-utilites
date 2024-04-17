mod login;

use clap::{Parser, Subcommand};
use login::login;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Gcli {
    /// Turn debuggin information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generates a login to google using the mobile and desktop apps auth flow. 
    /// See the following link if you wish more information about this specific auth flow: 
    /// https://developers.google.com/identity/protocols/oauth2/native-app
    Login,
}

impl Gcli {
    /// Will process the command argument and exit with the appropiate code.
    pub fn process(&self) -> i32 {
        match &self.command {
            None => 0,
            Some(command) => self.process_command(&command)
        }
    }

    fn process_command(&self, command: &Commands) -> i32 {
        use Commands::*;

        match command {
            Login => { login() }
        }
    }
}
