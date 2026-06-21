use std::io;

use betterdp::{
    client::start_client,
    host::{capture::test_capture, capture_xcap::test_capture_xcap, start_host},
};

fn main() {
    println!("Do you want to host or connect (h/c)?");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    match input {
        // "h" => {
        //     start_host();
        // }
        // "c" => {
        //     start_client();
        // }
        // "cap" => {
        //     test_capture();
        // }
        "xcap" => {
            test_capture_xcap();
        }
        _ => {
            println!("This option doesn't exist, use \'h\' or \'c\'.");
        }
    }
}
