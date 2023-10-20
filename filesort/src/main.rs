use std::env;

// For the initialized global variables
use lazy_static::lazy_static;

// To set the program sleep
use std::thread;
use std::time::Duration;
use std::fs;

use chrono::prelude::*; // Import the chrono crate

// To execute the command
use std::process::Command;


// Build the static variables
lazy_static! {
    static ref HOME: String = {
        env::var("HOME").expect("Could not get HOME environment variable")
    };

    static ref DOWNLOADS_PATH: String = {
        format!("{}/Downloads", &*HOME)
    };

    static ref PICTURES_PATH: String = {
        format!("{}/Pictures", &*HOME)
    };

    static ref PICTURE_FORMAT_FILES: Vec<String> = vec![
	"png".to_string(),
	"jpg".to_string(),
	"jpeg".to_string(),
	"gif".to_string(),
	"bmp".to_string(),
	"tiff".to_string(),
	"webp".to_string(),
	"svg".to_string(),
	// Add more formats as needed
    ];

    static ref DOCUMENTS_PATH: String = {
        format!("{}/Documents", &*HOME)
    };

    static ref DOCUMENT_FORMAT_FILES: Vec<String> = vec![
	"pdf".to_string(),
	"doc".to_string(),
	"docx".to_string(),
	"txt".to_string(),
	"rtf".to_string(),
	"odt".to_string(),
	"html".to_string(),
	"zip".to_string(),
	"rar".to_string(),
	"xlsx".to_string(),
	// Add more formats as needed
    ];

    static ref MUSIC_PATH: String = {
        format!("{}/Music", &*HOME)
    };

    static ref MUSIC_FORMAT_FILES: Vec<String> = vec![
	"mp3".to_string(),
	"flac".to_string(),
	"wav".to_string(),
	"ogg".to_string(),
	"m4a".to_string(),
	"aac".to_string(),
	// Add more formats as needed
    ];


    static ref VIDEOS_PATH: String = {
        format!("{}/Videos", &*HOME)
    };

    static ref VIDEO_FORMAT_FILES: Vec<String> = vec![
	"mp4".to_string(),
	"avi".to_string(),
	"mkv".to_string(),
	"mov".to_string(),
	"wmv".to_string(),
	"flv".to_string(),
	// Add more formats as needed
    ];
}


fn main()
{        
    // Runs infinitely the program
    loop {
        if let Ok(entries) = fs::read_dir(&*DOWNLOADS_PATH) {
	    
	    // Iterate through the files
            for entry in entries {
                if let Ok(entry) = entry {
		    let path = entry.path();
		    // Get the entry name
		    let entry_name = entry.file_name().to_string_lossy().to_string();
		    // Then get the time and formated
		    // let file_time = entry.metadata().modified(); 
		    let modified_datetime: DateTime<Utc> = entry.metadata().unwrap().modified().unwrap().into();
		    let datetime_formated = modified_datetime.format("%Y-%m-%d %H:%M:%S").to_string();

		    // Check if it is a file
		    if path.is_file() {
			println!("File Found: {:?} - Modified at: {}", entry_name, datetime_formated);
			
			// Split the file name into parts based on the dot (.) and try to detect 
			let parts: Vec<&str> = entry_name.split('.').collect();
			if parts.len() > 1 {
			    if let Some(file_format) = parts.last() {
				println!("File Format: {:?}", file_format);
				// Find where to put the file
				let mut path_to_move = "";
				if DOCUMENT_FORMAT_FILES.contains(&file_format.to_string()) {
				    path_to_move = &DOCUMENTS_PATH;
				} else if PICTURE_FORMAT_FILES.contains(&file_format.to_string()) {
				    path_to_move = &PICTURES_PATH;
				} else if MUSIC_FORMAT_FILES.contains(&file_format.to_string()) {
				    path_to_move = &MUSIC_PATH;
				} else if VIDEO_FORMAT_FILES.contains(&file_format.to_string()) {
				    path_to_move = &VIDEOS_PATH;
				}

				if path_to_move == "" {
				    println!("NOT KNOW WHERE TO MOVE");
				} else {
				    // Move to the path
				    println!("TO MOVE: {:?}", path_to_move);
				    let source_file_path = DOWNLOADS_PATH.to_string() + "/" + &entry_name;
				    let output = Command::new("mv")
					.arg(&source_file_path)
					.arg(path_to_move)
					.output()
					.expect("Failed to execute move command");
				    
				    if output.status.success() {
					println!("File moved successfully!");
				    } else {
					eprintln!("Failed to move the file. Error: {:?}", output.status);
				    }

				}
			    } else {
				println!("Unable to determine the file format.");
			    }
			} else {
			    println!("Unable to determine the file format.");
			}
                    } else {
			println!("Dir Found: {:?} - Modified at: {}", entry_name, datetime_formated);
		    }
		}
            }
        } else {
	    // If an error ocurred
            eprintln!("Failed to read the Downloads folder.");
        }

        // Sleep the program for 3 seconds
        thread::sleep(Duration::from_secs(3));
    }
}
