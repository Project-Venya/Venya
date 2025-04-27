use std::fmt::format;

use dioxus::prelude::*;
use text_to_speech::{kokoro::Kokoro, XdTts};

// The database is only available to server code
#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        // Open the database from the persisted "hotdog.db" file
        let conn = rusqlite::Connection::open("hotdog.db").expect("Failed to open database");

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

#[server]
pub async fn save_dog(image: String) -> Result<(), ServerFnError> {
    DB.with(|f| f.execute("INSERT INTO dogs (url) VALUES (?1)", &[&image]))?;
    Ok(())
}

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

pub async fn convert_text_to_speech(text: String) -> Result<(), Box<dyn std::error::Error>> {
    // Create a new instance of XdTts
    let text_to_speech_engine = XdTts::new(false)?;

    let cargo_path = env!("CARGO_MANIFEST_DIR");
    let output_spectrogram = format!("{}/assets/audios/output/mel_spectrogram.npy", cargo_path);
    let audio_output_file = format!("{}/assets/audios/output/audio_output.wav", cargo_path);

    println!("Intiated TTS");

    // Convert the text to speech
    text_to_speech_engine.generate_audio(
        text.as_str(),
        &audio_output_file,
        Some(std::path::PathBuf::from(output_spectrogram)),
    )?;

    Ok(())
}

pub fn initialize_text_to_speech() -> Result<Kokoro, Box<dyn std::error::Error>> {
    // Convert the text to speech
    let audio_output_file = format!(
        "{}/assets/audios/output/audio_output_kokoro.wav",
        env!("CARGO_MANIFEST_DIR")
    );

    // Create a new instance of Kokoro
    let kokoro_tts = Kokoro::new(audio_output_file);

    println!("Initiated TTS");

    Ok(kokoro_tts)
}
