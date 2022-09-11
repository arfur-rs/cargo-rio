use std::{
    fs,
    io::Write,
    net::TcpStream,
    os::unix::prelude::MetadataExt,
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

    /// A directory containing FRC libraries, that will be deployed under
    /// `/usr/local/frc/third-party/lib` in the RIO.
    #[clap(long, short)]
    library_dir: Option<PathBuf>,
}

impl DeploymentArgs {
    pub fn exec(self) -> Result<()> {
        let DeploymentArgs {
            team_number,
            address,
            executable,
            library_dir,
        } = self;

        let target = match (team_number, address) {
            (None, None) => unreachable!("clap groups should prevent this"),
            (None, Some(x)) => x,
            (Some(x), None) => cargo_rio::remote::find_rio_with_number(x)?,
            (Some(_), Some(_)) => unreachable!("clap groups should prevent this"),
        };

        trace!(target);

        let executable = match executable {
            Some(x) => x,
            None => todo!(),
        };

        let executable_size = std::fs::metadata(&executable)?.size();

        trace!(
            executable = format!("{executable:?}"),
            size = executable_size,
        );

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
            executable_size,
            None,
        )?;

        trace!("sending over scp...");

        remote_file.write_all(&std::fs::read(&executable)?)?;
        remote_file.send_eof()?;
        remote_file.wait_eof()?;
        remote_file.close()?;
        remote_file.wait_close()?;

        match library_dir {
            Some(library_dir) => {
                for library_file in fs::read_dir(library_dir)? {
                    let library_file = library_file?;
                    let library_file = library_file.path();

                    let size = fs::metadata(&library_file)?.size();
                    // TODO: research if this is the intended target directory.
                    let remote_path_name = format!(
                        "/lib/{library_file}",
                        library_file = library_file.file_name().unwrap().to_str().unwrap(),
                    );

                    let mut remote_file =
                        session.scp_send(Path::new(&remote_path_name), 0o777, size, None)?;

                    remote_file.write_all(&std::fs::read(&library_file)?)?;
                    remote_file.send_eof()?;
                    remote_file.wait_eof()?;
                    remote_file.close()?;
                    remote_file.wait_close()?;

                    println!(
                        "Sent library {library_file} succesfully...",
                        library_file = library_file.to_str().unwrap()
                    );
                }
            }
            None => (),
        }

        println!("Send complete ✅");

        Ok(())
    }
}
