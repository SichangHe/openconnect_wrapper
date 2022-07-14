use std::{error::Error, process::Command};

fn main() -> Result<(), Box<dyn Error>> {
    let mut process = Command::new("openconnect").arg("portal.duke.edu").spawn()?;
    process.wait()?;
    Ok(())
}
