// This is a comment, and will be ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->
use std::env;
// This is the main function
fn main() {
    // The statements here will be executed when the compiled binary is called

    // Print text to the console
    println!("Hello World2!");

    match env::var("TR_TORRENT_DIR") {
        Ok(dir) => println!("TR_TORRENT_DIR: {}", dir),
        Err(e) => println!("Couldn't read TR_TORRENT_DIR ({})", e),
    };

    // match env::var("TR_TORRENT_NAME") {
    //     Ok(name) => println!("TR_TORRENT_NAME: {}", name),
    //     Err(e) => println!("Couldn't read TR_TORRENT_NAME ({})", e),
    // };

    let name = env::var("TR_TORRENT_NAME").unwrap();
    println!("TR_TORRENT_NAME: {}", name);
}
