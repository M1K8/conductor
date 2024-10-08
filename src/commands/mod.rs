use std::io;

use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use clap_complete::{aot::Bash, generate};

#[derive(Subcommand, Clone, ValueEnum, Debug)]

pub(crate) enum Command {
    Interactive,
    Ping,
    Print,
    Upload,
}

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum Mode {
    Bambu { nested: Command },
    Klipper,
}
impl Mode {
    pub fn validate_necessary_args(&self, _c: &Cmd) -> Option<ArgError> {
        None
    }
}

#[derive(Parser, Clone)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub(crate) struct Cmd {
    /// What mode to run the program in
    #[command(subcommand)]
    pub mode: Option<Mode>,

    //todo - make the following opts, but unwrap at top lvl so values arent dropped
    #[arg(env("BAMBU_DEVICE"))]
    pub bbu_dev_id: Option<String>,

    #[arg(env("MQTT_SRV"))]
    pub mqtt_server: Option<String>,

    #[arg(env("FTP_USER"))]
    pub ftp_user: Option<String>,

    #[arg(env("FTP_PW"))]
    pub ftp_pw: Option<String>,

    #[arg(env("DEVICE_IP"))]
    pub dev_ip: Option<String>,
}

//TODO - decide when to use / how to bundle
fn _generate_tab_complete() {
    generate(Bash, &mut Cmd::command(), "testest", &mut io::stdout());
}

#[derive(Debug)]

pub(crate) struct ArgError {
    _missing_args: Vec<String>,
}
