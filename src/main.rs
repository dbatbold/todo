mod db;
mod config;

fn main() {
    let config = config::load_config("./todo.conf");
    let constr = config.get_must("postgres.constr");
    
    let mut conn = match db::connect(&constr) {
        Ok(c) => c,
        Err(e) => {
            println!("{:#?}", e);
            return;
        },
    };

    let rows = conn.query("SELECT * FROM list", &[]).unwrap();
    rows.iter().for_each(|row| println!("{}", row.get::<_, &str>(1)));
}
