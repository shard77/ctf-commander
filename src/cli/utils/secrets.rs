use keyring::{Entry, Error};

pub fn store_secret(
    entry_service: &str,
    entry_name: &str,
    credential: &str,
) -> keyring::Result<()> {
    let entry = Entry::new(entry_service, entry_name)?;
    entry.set_password(credential)?;
    Ok(())
}

pub fn retrieve_secret(entry_service: &str, entry_name: &str) -> Result<String, Error> {
    let entry = Entry::new(entry_service, entry_name)?;
    let result = entry.get_password()?;
    Ok(result)
}
