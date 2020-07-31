use postgres::{Client, NoTls};

use std::fs;

pub fn connect(conf_file: &str) -> Result<Client, postgres::Error> {
    let constr = fs::read_to_string(conf_file).unwrap();
    let mut conn = Client::connect(&constr, NoTls)?;
    Ok(conn)
}
