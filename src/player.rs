use rodio::{Decoder, OutputStream, Sink};
use std::fs;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::network::InternetConnection;

pub fn list_songs() {
    let sangfiler = fs::read_dir("music").expect("Der skal være filer i 'music'");

    let mut a = true;
    for (i, entry) in sangfiler.enumerate() {
        let sang = entry.expect("Fil skal være læsbar").path();
        if sang.extension().unwrap() == ("mp3") {
            println!("{}. {:?}", i + 1, sang.file_name().unwrap());
            a = false;
        }
    }

    if a {
        println!("Der skal være filer i 'music'");
    }
}


// Skrevet af AI - Er ikke blevet skrevet "ordentligt" - men det virker.
pub fn play_song(index: usize, internet: Arc<Mutex<InternetConnection>>) {
    let songs: Vec<_> =
        fs::read_dir("music").expect("Der skal være filer i 'music'")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|s| s.to_str()) == Some("mp3"))
        .collect();

    if index == 0 || index > songs.len() {
        println!("Ugyldigt sangnummer!");
        return;
    }

    let song_path = &songs[index - 1];
    println!("▶️ Afspiller: {:?}", song_path.file_name().unwrap());

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = std::fs::File::open(song_path).expect("Kunne ikke åbne fil");
    let source = Decoder::new(BufReader::new(file)).unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    sink.append(source);
    sink.play();

    loop {
        thread::sleep(Duration::from_secs(1));

        let net = internet.lock().unwrap();
        if !net.is_connected {
            println!("Internetforbindelse afbrudt! Musikken pauses...");
            sink.pause();
            break;
        }
    }
}
