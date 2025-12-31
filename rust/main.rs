// Rust comes with build system and package manager called "cargo"

// cargo new "name"
// creates a new folder
//   Cargo.toml (like package.json)
//   src contains source files (it's own "Hello, world!")

// cd cargo && cargo build -> generates a target/ folder, and Cargo.lock
// cargo run -> prints "hello world"

// cargo check -> checks for errors without executable


// fn main serves as the entrypoint to the program
fn main() {
    println!("hello rust"); // ! is a macro
}
