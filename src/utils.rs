use std::io::{self, Write};
use std::thread;
use std::time::Duration;


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



//TODO - ubibliotecznić to sprytnie
pub fn spawn<F, T>(name: String, block: F)
    where F: FnOnce() -> T + Send + Sync + 'static, T: Send + Sync + 'static {

    
    let result = thread::Builder::new().name(name.clone()).spawn(block);
        
    match result {
        Ok(_) => {},
        Err(err) => panic!("Can't spawn {}: {}", name, err),
    };
}


pub fn sleep(time: u64) {
    thread::sleep(Duration::from_millis(time));
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

/*
// Adds prefix and suffix to text to make it red (0;31)
macro_rules! text_red {
    ($fmt:expr) => { concat!("\x1B[0;31m", $fmt, "\x1B[m") };
}

// Adds prefix and suffix to text to make it bold and red (1;31)
macro_rules! text_bold_red {
    ($fmt:expr) => { concat!("\x1B[1;31m", $fmt, "\x1B[m") };
}

macro_rules! log_crit {
    ($fmt:expr) => ({
        use std::io::Write;
        match writeln!(&mut ::std::io::stderr(), text_bold_red!(concat!("CRITICAL: ", $fmt))) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    });

    ($fmt:expr, $($arg:tt)*) => ({
        use std::io::Write;
        match writeln!(&mut ::std::io::stderr(), text_bold_red!(concat!("CRITICAL: ", $fmt)), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    });
}

macro_rules! log_error {
    ($fmt:expr) => ({
        use std::io::Write;
        match writeln!(&mut ::std::io::stderr(), text_red!(concat!("ERROR: ", $fmt))) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    });

    ($fmt:expr, $($arg:tt)*) => ({
        use std::io::Write;
        match writeln!(&mut ::std::io::stderr(), text_red!(concat!("ERROR: ", $fmt)), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    });
}
*/
//macro_rules! warn {
//}

//macro_rules! info {
//}


