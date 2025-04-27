use dioxus::prelude::*;
use speech_to_text::AudioRecorder;

use crate::backend::initialize_text_to_speech;

#[component]
pub fn SpeechToText() -> Element {
    let mut is_recording = use_signal(|| false);
    let mut button_label = use_signal(|| "Start Recording".to_string());
    let mut button_color = use_signal(|| "!bg-blue-500 hover:bg-blue-700".to_string());

    let mut translated_text = use_signal(|| Vec::new());

    let cargo_path = env!("CARGO_MANIFEST_DIR");

    let mut audio_recorder = use_signal(|| AudioRecorder::new(cargo_path).unwrap());
    let mut text_to_speech_engine = use_signal(|| initialize_text_to_speech().unwrap());

    let mut text_to_speak = use_signal(|| {
        "Venya is a voice-first AI assistant that listens, learns, and grows with you — helping you manage your day, reflect, read books, and spark conversations.".to_string()
    });

    let toggle_recording = move |_event| async move {
        if is_recording() {
            _ = audio_recorder.write().stop().unwrap();
            is_recording.set(false);
            button_label.set("Start Recording".to_string());
            button_color.set("!bg-blue-500 hover:bg-blue-700".to_string());
        } else {
            _ = audio_recorder.write().start().unwrap();
            is_recording.set(true);
            button_label.set("Stop Recording".to_string());
            button_color.set("!bg-red-500 hover:bg-red-700".to_string());
        }
    };

    let convert_to_text = move |_event| async move {
        if !is_recording() {
            // let response_text = audio_recorder
            //     .write()
            //     .convert_speech_to_text_vosk()
            //     .unwrap();

            // println!("{:?}", response_text);

            let response_whisper = audio_recorder
                .write()
                .convert_speech_to_text_whisper()
                .unwrap();

            translated_text.set(response_whisper.clone());

            println!("{:#?}", response_whisper);
        }
    };

    let speak_text = move |_event| async move {
        // let _ = convert_text_to_speech(text_to_speak()).await.unwrap();

        println!("Starting....");

        let _ = text_to_speech_engine
            .write()
            .generate_audio(text_to_speak().as_str());

        println!("Speaking: {}", text_to_speak.read());
    };

    rsx! {
        div {
            class: "h-full w-full flex flex-col items-center justify-center space-y-4",
            div {
                class: "text-2xl font-bold",
                "Experiments"
            }
            div {
                class: "text-base font-semibold",
                "Text to Speech"
            }
            select {
                class: "w-64 border-2 border-gray-300 rounded-md p-2",
                onchange: move |event| {
                    let selected_index = event.to_owned().value().parse::<usize>().unwrap_or(0);
                    text_to_speech_engine.write().set_speaker(selected_index as u32);
                },
                for (index, speaker) in text_to_speech_engine.read().get_speakers().iter().enumerate() {
                    option {
                        value: "{index}",
                        selected: text_to_speech_engine.read().current_speaker == index as u32,
                        "{speaker.name} ({speaker.gender})"
                    }
                }
            }
            textarea {
                class: "w-64 h-32 border-2 border-gray-300 rounded-md p-2",
                placeholder: "Type text to speak...",
                value: "{text_to_speak}",
                oninput: move |event| {
                    text_to_speak.set(event.to_owned().value());
                }
            }
            button {
                onclick: speak_text,
                class: "bg-purple-500 hover:bg-purple-700 text-white cursor-pointer font-normal py-2 px-4 rounded",
                "Speak Text"
            }
            div {
                class: "text-base font-semibold pt-3",
                "Speech to Text"
            }
            button {
                onclick: toggle_recording,
                class: format!("{} text-white cursor-pointer font-normal py-2 px-4 rounded", button_color()),
                "{button_label}"
            }
            button {
                onclick: convert_to_text,
                class: "bg-green-500 hover:bg-green-700 text-white cursor-pointer font-normal py-2 px-4 rounded",
                "Convert to Text"
            }
            div {
             class: "text-base pt-4 px-5",
             for segment in translated_text().iter() {
                 div {
                     margin_bottom: "1rem",
                     strong {"{segment.start_time.split('.').next().unwrap_or_default()} - {segment.end_time.split('.').next().unwrap_or_default()}:"}
                     p {"{segment.content}"}
                 }
             }
            }
        }
    }
}
