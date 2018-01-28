// This is a comment, and will be ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->
use std::env;
use std::path::Path;
use std::process::Command;

fn move_file(from_file: &Path) {
    Command::new("mv")
        .arg(from_file.to_str().unwrap())
        .arg("/home/andreip/")
        .spawn()
        .expect("mv command failed to start");
}

// This is the main function
fn main() {
    let env_name_result = env::var("TR_TORRENT_NAME");
    let name = env_name_result.unwrap();
    let dir = env::var("TR_TORRENT_DIR").unwrap();
    println!("TR_TORRENT_NAME: {}", name);
    println!("TR_TORRENT_DIR: {}", dir);

    let env_torrent_name = "TR_TORRENT_NAME";
    match env::var(env_torrent_name) {
        Ok(torrent_name) => torrent_location.push_str(torrent_name.as_str()),
        Err(e) => println!("couldn't interpret {}: {}", env_torrent_name, e),
    }
    println!("Location: {:?}", torrent_location);
    
    let path = Path::new(&torrent_location);
    if path.is_dir() {
        println!("Yay dir");
    } else if path.is_file() {
        move_file(path);
        // println!("Yay file");
        // Command::new("ls")i
        //     .arg(path)
        //     .arg("-l")
        //     .arg("-a")
        //     .spawn()
        //     .expect("ls command failed to start");
    } else {
        panic!("explicit panic");
    }
}
