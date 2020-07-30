use postgres::{Client, NoTls};

pub fn connect() -> Result<Client, postgres::Error> {
    let constr = "host=localhost user=todo";
    let mut conn = Client::connect(constr, NoTls)?;
    Ok(conn)
}
