mod db;

fn main() {
    let mut conn = match db::connect("todo.conf") {
        Ok(c) => c,
        Err(e) => {
            println!("{:#?}", e);
            return;
        },
    };

    let rows = conn.query("SELECT * FROM list", &[]).unwrap();
    rows.iter().for_each(|row| println!("{}", row.get::<_, &str>(1)));
}
