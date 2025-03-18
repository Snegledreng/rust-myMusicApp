mod network;
mod player;

use std::io;
use std::sync::{Arc, Mutex};
use player::{list_songs, play_song};
use network::InternetConnection;

fn main() {
    let internet = Arc::new(Mutex::new(InternetConnection::new()));

    loop {
        println!("      Musikstreaming App");
        println!("1. Afspil en sang");
        println!("2. Toggle internetforbindelse");
        println!("3. Afslut");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice);

        match choice.trim() {
            "1" => {
                list_songs();
                println!("Indtast nummeret på sangen:");
                let mut song_choice = String::new();
                io::stdin().read_line(&mut song_choice);

                if let Ok(song_index) = song_choice.trim().parse::<usize>() {
                    // Vil gerne have play_song kører på en ny tråd, så jeg kan fortsætte loop imens, for f.eks. at kunne toggle internet
                    play_song(song_index, Arc::clone(&internet));
                } else {
                    println!("Ugyldigt nummer");
                }
            }
            "2" => { internet.lock().unwrap().toggle(); }
            "3" => { break; }
            _ => {}
        }
    }
}
