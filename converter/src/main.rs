use regex::Regex;
use rusqlite::Connection;
use std::path::Path;

fn main() -> Result<(), ()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        println!("Please give a file path");
        return Err(());
    }
    let file = &args[1];
    if !Path::new(file).exists() {
        println!("\"{file}\" is not a valid file");
        return Err(());
    }
    run(file).unwrap().close().unwrap();
    Ok(())
}
fn run(file: &String) -> rusqlite::Result<Connection> {
    // Open new database
    let db = Connection::open(format!(
        "{}/Documents/songs.sqlite",
        std::env::var("HOME").expect("The HOME variable is not defined")
    ))?;

    // Open Open-LP database
    db.execute("ATTACH DATABASE ? AS old;", &[(file)])?;
    db.execute_batch(
        "CREATE TABLE IF NOT EXISTS songs (
                id     INTEGER PRIMARY KEY AUTOINCREMENT,
                title  VARCHAR(255),
                lyrics TEXT NOT NULL,
                book   INTEGER,
                number INTEGER
        );
        CREATE TABLE IF NOT EXISTS authors (
                id     INTEGER PRIMARY KEY,
                name   VARCHAR(255)
        );
        CREATE TABLE IF NOT EXISTS authors_songs (
                author_id INTEGER NOT NULL,
                song_id   INTEGER NOT NULL
        );
        CREATE TABLE IF NOT EXISTS books (
                id   INTEGER PRIMARY KEY,
                name VARCHAR(255)
        );
        INSERT INTO songs (title,lyrics) SELECT title,lyrics FROM old.songs;
        INSERT INTO books (name) VALUES ('JEM'), ('JEMK'), ('ATG');
        INSERT INTO authors (id,name) SELECT id,display_name FROM old.authors;

        INSERT INTO authors_songs (author_id,song_id)
        SELECT oas.author_id,s.id
        FROM old.authors_songs oas
        JOIN old.songs os ON oas.song_id = os.id
        JOIN songs s ON os.title = s.title;
        ",
    )?;

    // Set the songbook id of each song (based on the old song title)
    let mut number = db.prepare("UPDATE songs SET book = ? WHERE title LIKE ?;")?;
    number.execute(["1", "<JEM %>  %"])?;
    number.execute(["2", "<JEMK %>  %"])?;
    number.execute(["3", "<ATG %>  %"])?;
    number.finalize()?;

    // Remove songbook and number from the beginning of the title
    let mut query = db.prepare("SELECT title FROM songs WHERE book IS NOT NULL;")?;
    let result = query.query_map([], |row| Ok(row.get::<_, String>(0)?))?; // get the titles to change
    let regex = Regex::new(r"<[A-Z]+ ([0-9]+)>  (.+)").unwrap();
    let mut update =
        db.prepare_cached("UPDATE songs SET title = ?, number = ? WHERE title = ?;")?; // request to change title

    let start = std::time::Instant::now();
    for i in result {
        let title = i?;
        if let Some(split) = regex.captures(&title) {
            update.execute((
                split.get(2).unwrap().as_str(),
                split.get(1).unwrap().as_str(),
                &title,
            ))?; // change the title
        }
    }
    let stop = start.elapsed();
    println!("Changing songbook number takes {} ms.", stop.as_millis());
    query.finalize()?;
    update.discard();

    db.flush_prepared_statement_cache();
    Ok(db)
}
