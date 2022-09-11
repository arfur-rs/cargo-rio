use color_eyre::Result;
use thiserror::Error;

pub fn find_rio_with_number(team_number: u16) -> Result<String> {
    // TODO: factor in support for 5-digit teams.

    let (first, second) = {
        let stringified = team_number.to_string();
        let first: String = stringified.chars().take(2).collect();
        let second: String = stringified.chars().skip(2).collect();
        (first, second)
    };

    let mut possible_addresses = vec![
        "172.22.11.2".to_string(),
        format!("roborio-{team_number}-FRC.local"),
        format!("10.{first}.{second}.2"),
    ]
    .into_iter();

    let address: Option<String> = loop {
        match possible_addresses.next() {
            Some(address) => {
                // Try the address. If it fails, no biggie. If it succeeds,
                // break with the address name.

                // Note that currently, we are just returning the first address.
                // TODO: Ping each address.
                break Some(address);
            }
            None => break None,
        };
    };

    address.ok_or(RemoteError::RIONotFound.try_into()?)
}

#[derive(Error, Debug)]
pub enum RemoteError {
    #[error("could not find a connected RIO")]
    RIONotFound,
}
