use std::io::{self, Write};
use std::thread;


pub fn log_error(message: String) {

    let stderr = io::stderr();
    let mut handle = stderr.lock();

    //handle.write(b"hello world").unwrap();
    handle.write(format!("\x1B[1;31m{:<20}: {}\x1B[m\n", thread::current().name().unwrap_or("<unnamed>"), message).as_bytes()).unwrap();

    //show(message) - trzeba na czerwono wyświeltić ten komuniakt - ale tylko w przypadku
    //  wyswietlania na ekran, bo do pliku albo do sysloga to be zkoloryzowania
}


pub fn log_info(message: String) {
    println!("\x1B[32m{:<20}: {}\x1B[39m", thread::current().name().unwrap_or("<unnamed>"), message);
}

pub fn log_debug(message: String) {
    println!("{:<20}: {}", thread::current().name().unwrap_or("<unnamed>"), message);
}


// https://en.wikipedia.org/wiki/Syslog#Severity_level
/*
enum Level {
    Emerg = 0,
    Alert,
    Crit,
    Error,
    Warn,
    Notice,
    Info,
    Debug
}*/

// http://stackoverflow.com/questions/27588416/how-to-send-output-to-stderr



