use sherpa_rs::tts::{KokoroTts, KokoroTtsConfig};

pub struct Speaker {
    pub id: u32,
    pub name: String,
    pub gender: String,
    pub native_language: String,
}

pub struct Kokoro {
    tts: KokoroTts,
    pub speakers: Vec<Speaker>,
    pub speed: f32,
    pub current_speaker: u32,
    pub audio_output_file: String,
}

impl Kokoro {
    pub fn new(audio_output_file: String) -> Self {
        let cargo_manifest_path = env!("CARGO_MANIFEST_DIR");

        let model_path = format!("{}/models/kokoro-multi-lang-v1_0", cargo_manifest_path);
        let config = KokoroTtsConfig {
            model: format!("{}/model.onnx", model_path),
            voices: format!("{}/voices.bin", model_path),
            tokens: format!("{}/tokens.txt", model_path),
            data_dir: format!("{}/espeak-ng-data", model_path),
            dict_dir: format!("{}/dict", model_path),
            lexicon: format!(
                "{}/lexicon-us-en.txt,{}/lexicon-zh.txt",
                model_path, model_path
            ),
            length_scale: 1.0,
            ..Default::default()
        };

        let tts = KokoroTts::new(config);

        let new_instance = Self {
            tts,
            speakers: set_available_speakers(),
            speed: 1.0,
            current_speaker: 0,
            audio_output_file,
        };

        new_instance
    }

    pub fn generate_audio(&mut self, text: &str) {
        let sid = self.speakers[self.current_speaker as usize].id as i32;

        let audio = self.tts.create(&text, sid, self.speed).unwrap();

        sherpa_rs::write_audio_file(
            &self.audio_output_file.as_str(),
            &audio.samples,
            audio.sample_rate,
        )
        .unwrap();
    }

    pub fn get_speakers(&self) -> &Vec<Speaker> {
        &self.speakers
    }

    pub fn set_speaker(&mut self, index: u32) {
        self.current_speaker = index;
    }
}

fn set_available_speakers() -> Vec<Speaker> {
    vec![
        // American English
        Speaker {
            id: 0,
            name: "Heart".to_string(),
            gender: "Female".to_string(),
            native_language: "American English".to_string(),
        },
        Speaker {
            id: 1,
            name: "Alloy".to_string(),
            gender: "Female".to_string(),
            native_language: "American English".to_string(),
        },
        Speaker {
            id: 2,
            name: "Aoede".to_string(),
            gender: "Female".to_string(),
            native_language: "American English".to_string(),
        },
        Speaker {
            id: 3,
            name: "Bella".to_string(),
            gender: "Female".to_string(),
            native_language: "American English".to_string(),
        },
        Speaker {
            id: 4,
            name: "Jessica".to_string(),
            gender: "Female".to_string(),
            native_language: "American English".to_string(),
        },
        Speaker {
            id: 5,
            name: "Kore".to_string(),
            gender: "Female".to_string(),
            native_language: "American English".to_string(),
        },
        Speaker {
            id: 6,
            name: "Nicole".to_string(),
            gender: "Female".to_string(),
            native_language: "American English".to_string(),
        },
        Speaker {
            id: 7,
            name: "Nova".to_string(),
            gender: "Female".to_string(),
            native_language: "American English".to_string(),
        },
        Speaker {
            id: 8,
            name: "River".to_string(),
            gender: "Female".to_string(),
            native_language: "American English".to_string(),
        },
        Speaker {
            id: 9,
            name: "Sarah".to_string(),
            gender: "Female".to_string(),
            native_language: "American English".to_string(),
        },
        Speaker {
            id: 10,
            name: "Sky".to_string(),
            gender: "Female".to_string(),
            native_language: "American English".to_string(),
        },
        Speaker {
            id: 11,
            name: "Adam".to_string(),
            gender: "Male".to_string(),
            native_language: "American English".to_string(),
        },
        Speaker {
            id: 12,
            name: "Echo".to_string(),
            gender: "Male".to_string(),
            native_language: "American English".to_string(),
        },
        Speaker {
            id: 13,
            name: "Eric".to_string(),
            gender: "Male".to_string(),
            native_language: "American English".to_string(),
        },
        Speaker {
            id: 14,
            name: "Fenrir".to_string(),
            gender: "Male".to_string(),
            native_language: "American English".to_string(),
        },
        Speaker {
            id: 15,
            name: "Liam".to_string(),
            gender: "Male".to_string(),
            native_language: "American English".to_string(),
        },
        Speaker {
            id: 16,
            name: "Michael".to_string(),
            gender: "Male".to_string(),
            native_language: "American English".to_string(),
        },
        Speaker {
            id: 17,
            name: "Onyx".to_string(),
            gender: "Male".to_string(),
            native_language: "American English".to_string(),
        },
        Speaker {
            id: 18,
            name: "Puck".to_string(),
            gender: "Male".to_string(),
            native_language: "American English".to_string(),
        },
        Speaker {
            id: 19,
            name: "Santa".to_string(),
            gender: "Male".to_string(),
            native_language: "American English".to_string(),
        },
        // British English
        Speaker {
            id: 20,
            name: "Alice".to_string(),
            gender: "Female".to_string(),
            native_language: "British English".to_string(),
        },
        Speaker {
            id: 21,
            name: "Emma".to_string(),
            gender: "Female".to_string(),
            native_language: "British English".to_string(),
        },
        Speaker {
            id: 22,
            name: "Isabella".to_string(),
            gender: "Female".to_string(),
            native_language: "British English".to_string(),
        },
        Speaker {
            id: 23,
            name: "Lily".to_string(),
            gender: "Female".to_string(),
            native_language: "British English".to_string(),
        },
        Speaker {
            id: 24,
            name: "Daniel".to_string(),
            gender: "Male".to_string(),
            native_language: "British English".to_string(),
        },
        Speaker {
            id: 25,
            name: "Fable".to_string(),
            gender: "Male".to_string(),
            native_language: "British English".to_string(),
        },
        Speaker {
            id: 26,
            name: "George".to_string(),
            gender: "Male".to_string(),
            native_language: "British English".to_string(),
        },
        Speaker {
            id: 27,
            name: "Lewis".to_string(),
            gender: "Male".to_string(),
            native_language: "British English".to_string(),
        },
        // Japanese
        Speaker {
            id: 28,
            name: "Alpha".to_string(),
            gender: "Female".to_string(),
            native_language: "Japanese".to_string(),
        },
        Speaker {
            id: 29,
            name: "Gongitsune".to_string(),
            gender: "Female".to_string(),
            native_language: "Japanese".to_string(),
        },
        Speaker {
            id: 30,
            name: "Nezumi".to_string(),
            gender: "Female".to_string(),
            native_language: "Japanese".to_string(),
        },
        Speaker {
            id: 31,
            name: "Tebukuro".to_string(),
            gender: "Female".to_string(),
            native_language: "Japanese".to_string(),
        },
        Speaker {
            id: 32,
            name: "Kumo".to_string(),
            gender: "Male".to_string(),
            native_language: "Japanese".to_string(),
        },
        // Mandarin Chinese
        Speaker {
            id: 33,
            name: "Xiaobei".to_string(),
            gender: "Female".to_string(),
            native_language: "Mandarin Chinese".to_string(),
        },
        Speaker {
            id: 34,
            name: "Xiaoni".to_string(),
            gender: "Female".to_string(),
            native_language: "Mandarin Chinese".to_string(),
        },
        Speaker {
            id: 35,
            name: "Xiaoxiao".to_string(),
            gender: "Female".to_string(),
            native_language: "Mandarin Chinese".to_string(),
        },
        Speaker {
            id: 36,
            name: "Xiaoyi".to_string(),
            gender: "Female".to_string(),
            native_language: "Mandarin Chinese".to_string(),
        },
        Speaker {
            id: 37,
            name: "Yunjian".to_string(),
            gender: "Male".to_string(),
            native_language: "Mandarin Chinese".to_string(),
        },
        Speaker {
            id: 38,
            name: "Yunxi".to_string(),
            gender: "Male".to_string(),
            native_language: "Mandarin Chinese".to_string(),
        },
        Speaker {
            id: 39,
            name: "Yunxia".to_string(),
            gender: "Male".to_string(),
            native_language: "Mandarin Chinese".to_string(),
        },
        Speaker {
            id: 40,
            name: "Yunyang".to_string(),
            gender: "Male".to_string(),
            native_language: "Mandarin Chinese".to_string(),
        },
        // Spanish
        Speaker {
            id: 41,
            name: "Dora".to_string(),
            gender: "Female".to_string(),
            native_language: "Spanish".to_string(),
        },
        Speaker {
            id: 42,
            name: "Alex".to_string(),
            gender: "Male".to_string(),
            native_language: "Spanish".to_string(),
        },
        Speaker {
            id: 43,
            name: "Santa".to_string(),
            gender: "Male".to_string(),
            native_language: "Spanish".to_string(),
        },
        // French
        Speaker {
            id: 44,
            name: "Siwis".to_string(),
            gender: "Female".to_string(),
            native_language: "French".to_string(),
        },
        // Hindi
        Speaker {
            id: 45,
            name: "Alpha".to_string(),
            gender: "Female".to_string(),
            native_language: "Hindi".to_string(),
        },
        Speaker {
            id: 46,
            name: "Beta".to_string(),
            gender: "Female".to_string(),
            native_language: "Hindi".to_string(),
        },
        Speaker {
            id: 47,
            name: "Omega".to_string(),
            gender: "Male".to_string(),
            native_language: "Hindi".to_string(),
        },
        Speaker {
            id: 48,
            name: "Psi".to_string(),
            gender: "Male".to_string(),
            native_language: "Hindi".to_string(),
        },
        // Italian
        Speaker {
            id: 49,
            name: "Sara".to_string(),
            gender: "Female".to_string(),
            native_language: "Italian".to_string(),
        },
        Speaker {
            id: 50,
            name: "Nicola".to_string(),
            gender: "Male".to_string(),
            native_language: "Italian".to_string(),
        },
        // Brazilian Portuguese
        Speaker {
            id: 51,
            name: "Dora".to_string(),
            gender: "Female".to_string(),
            native_language: "Brazilian Portuguese".to_string(),
        },
        Speaker {
            id: 52,
            name: "Alex".to_string(),
            gender: "Male".to_string(),
            native_language: "Brazilian Portuguese".to_string(),
        },
        Speaker {
            id: 53,
            name: "Santa".to_string(),
            gender: "Male".to_string(),
            native_language: "Brazilian Portuguese".to_string(),
        },
    ]
}
