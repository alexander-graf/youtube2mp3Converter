
use fltk::{prelude::*, *};
use std::process::{Command, exit};
use std::str;
use std::thread;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;


fn main() {
    
    // holt sich die URL aus der Zwischenablage
    let url = get_clipboard_text();

    let a = app::App::default();

    let mut win = window::Window::default().with_size(600, 400);
    let mut pack = group::Pack::default().with_size(400, 400).center_of_parent();
    pack.set_spacing(10);
    pack.set_type(group::PackType::Vertical);

    let mut eingabe = input::Input::default().with_size(390, 30);
    pack.add(&eingabe);
    eingabe.set_value(&url);

    let mut convert_mp3_button = button::Button::default()
        .with_label("MP3_aktuell")
        .with_size(40, 30); // Specify the custom width and height here

    convert_mp3_button.set_callback(move |_| {
        let text_value = eingabe.value();
        println!("input: {}", text_value);
        thread::spawn(move || {
            convert_to_mp3(&text_value);
        
        });
    });
    pack.add(&convert_mp3_button); // Add the button

    win.end();
    win.show();

    a.run().unwrap();
}

fn convert_to_mp3(url: &str) {
    let yt_dlp_path = "C:/youtube/yt-dlp.exe";
    let output_dir = "C:/youtube/mp3/";

    println!("URL: {}", url);

    // Extract the video title and channel name using youtube-dl
    let info_output = Command::new(yt_dlp_path)
        .args(&["--get-title", "--get-filename", &url])
        .output()
        .expect("Failed to execute youtube-dl command");
    
    println!("output: {:?}", info_output);

    let info = str::from_utf8(&info_output.stdout)
        .unwrap_or("output")
        .trim();

    let mut info_lines = info.lines();
    let title = info_lines.next().unwrap_or("");
    let filename = info_lines.next().unwrap_or("");

    let channel_name = filename.rsplitn(2, '-').next().unwrap_or("").trim();

    let output_path = format!("{}{} - {}.mp3", output_dir, title, channel_name);

    let output = Command::new(yt_dlp_path)
        .args(&["-x", "--audio-format", "mp3", "-o", &output_path, &url])
        .output();

    match output {
        Ok(_) => {
            println!("Conversion successful!");
            // Your logic after successful conversion
        }
        Err(e) => {
            eprintln!("Error during conversion: {:?}", e);
            exit(1);
        }
    }
}


fn get_clipboard_text() -> String {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    if let Ok(text) = ctx.get_contents() {
        return text;
    }
    String::new()
}