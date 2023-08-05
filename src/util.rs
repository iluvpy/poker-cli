use std::io::Write;
use std::io;


pub fn flush_stdout() {
    let _ = io::stdout().flush().unwrap();
}
