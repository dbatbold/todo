mod db;

fn main() {
    let mut conn = match db::connect() {
        Ok(c) => c,
        Err(e) => {
            println!("{:#?}", e);
            return;
        },
    };

    let _ = conn.execute("SELECT * FROM todo", &[]);
}
