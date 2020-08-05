use postgres::{Client, NoTls};

pub fn connect(constr: &str) -> Result<Client, postgres::Error> {
    let conn = Client::connect(constr, NoTls)?;
    Ok(conn)
}
