use rusty_audio::Audio;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();

    let audio_files_path = std::fs::read_dir("./sounds").unwrap();

    // Can be used to file audio.add
    for file in audio_files_path {
        println!("Name: {}", file.unwrap().path().display());
    }

    audio.add("explode", "./sounds/explode.wav");
    audio.add("lose", "./sounds/lose.wav");
    audio.add("move", "./sounds/move.wav");
    audio.add("pew", "./sounds/pew.wav");
    audio.add("startup", "./sounds/startup.wav");
    audio.add("win", "./sounds/win.wav");
    
    audio.play("startup");
    audio.wait();
    Ok(())
}

