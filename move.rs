// This is a comment, and will be ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->
use std::env;
use std::path::Path;
// This is the main function
fn main() {
    let name = env::var("TR_TORRENT_NAME").unwrap();
    let dir = env::var("TR_TORRENT_DIR").unwrap();
    println!("TR_TORRENT_NAME: {}", name);
    println!("TR_TORRENT_DIR: {}", dir);

    let result = [dir, name].join("/");
    println!("result: {}", result);
    
    let path = Path::new(&result);
    if path.isDir() {
        println!("Yay dir");
    } else {
        println!("Nay dir");
    }
}
