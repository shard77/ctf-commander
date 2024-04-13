use rpassword::read_password;
use std::io;

pub fn input_credentials() -> Result<String, io::Error> {
    let credential = read_password()?;
    Ok(credential)
}
