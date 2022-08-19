use std::{
    fs::File,
    io::{Read, Write},
    net::TcpStream,
    path::{Path, PathBuf},
};

use clap::Args;
use color_eyre::Result;
use ssh2::Session;
use tracing::trace;

#[derive(Args, Debug)]
#[clap(version)]
#[clap(group = clap::ArgGroup::new("addr").required(true).multiple(false))]
pub struct DeploymentArgs {
    /// Your team number.
    #[clap(long, group = "addr")]
    team_number: Option<u16>,

    /// The RoboRIO target address.
    #[clap(long, short, group = "addr")]
    address: Option<String>,

    /// The path to the executable.
    #[clap(long, short)]
    executable: Option<PathBuf>,
}

impl DeploymentArgs {
    pub fn exec(self) -> Result<()> {
        let DeploymentArgs {
            team_number,
            address,
            executable,
        } = self;

        let target = match (team_number, address) {
            (None, None) => unreachable!("clap groups should prevent this"),
            (None, Some(x)) => x,
            (Some(x), None) => cargo_rio::remote::find_rio_with_number(x)?,
            (Some(_), Some(_)) => unreachable!("clap groups should prevent this"),
        };

        trace!(target);

        let mut executable = match executable {
            Some(x) => File::open(x)?,
            None => todo!(),
        };

        trace!(executable = format!("{executable:?}"));

        let mut executable_contents = Vec::new();
        executable.read_to_end(&mut executable_contents)?;

        let tcp = TcpStream::connect(format!("{target}:22"))?;
        let mut session = Session::new()?;
        session.set_tcp_stream(tcp);
        session.handshake()?;
        session.userauth_password("lvuser", "")?;

        println!("SSH connection to RIO established at {target}:22.");

        trace!(
            "SSH connection authenticated: {authenticated}",
            authenticated = session.authenticated()
        );

        let mut remote_file = session.scp_send(
            Path::new("/home/lvuser/robotCommand"),
            0o777,
            executable_contents.len() as u64,
            None,
        )?;

        trace!("sending over scp...");

        remote_file.write(&executable_contents)?;
        remote_file.send_eof()?;
        remote_file.wait_eof()?;
        remote_file.close()?;
        remote_file.wait_close()?;

        println!("Send complete ✅");

        Ok(())
    }
}
