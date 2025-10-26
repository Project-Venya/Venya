use dioxus::prelude::*;
use text_to_speech::kokoro::Kokoro;

/// The database is only available to server code
#[cfg(feature = "server")]
thread_local! {
    /// The SQLite connection to the database.
    pub static DB: rusqlite::Connection = {
        // Open the database from the persisted "hotdog.db" file
        let conn = rusqlite::Connection::open("hotdog.db").expect("Failed to open open database");

        // Create the "dogs" table if it doesn't already exist
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS dogs (
                id INTEGER PRIMARY KEY,
                url TEXT NOT NULL
            );",
        ).unwrap();

        // Return the connection
        conn
    };
}

/// Saves a dog image URL to the database.
#[server]
pub async fn save_dog(image: String) -> Result<(), ServerFnError> {
    DB.with(|f| f.execute("INSERT INTO dogs (url) VALUES (?1)", &[&image]))?;
    Ok(())
}

/// Lists the last 10 saved dog image URLs from the database.
#[server]
pub async fn list_dogs() -> Result<Vec<(usize, String)>, ServerFnError> {
    let dogs = DB.with(|f| {
        f.prepare("SELECT id, url FROM dogs ORDER BY id DESC LIMIT 10")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    });

    Ok(dogs)
}

/// Initializes the text-to-speech engine.
pub async fn initialize_text_to_speech() -> Result<Kokoro, Box<dyn std::error::Error>> {
    // Convert the text to speech
    let audio_output_file = format!(
        "{}/assets/audios/output/audio_output_kokoro.wav",
        env!("CARGO_MANIFEST_DIR")
    );

    // Create a new instance of Kokoro
    let kokoro_tts = tokio::task::spawn_blocking(move || Kokoro::new(audio_output_file))
        .await
        .unwrap();

    println!("Initiated TTS");

    Ok(kokoro_tts)
}
