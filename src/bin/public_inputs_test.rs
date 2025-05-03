use reqwest::blocking::get;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read};

fn main() {
    println!("--- Simple Client Starting ---");

    let seed_url = "http://127.0.0.1:8000/seed_bytes";
    println!("Fetching seed bytes from: {}", seed_url);

    let seed_response = match get(seed_url) {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("Error fetching seed bytes: {}", e);
            return;
        }
    };

    if !seed_response.status().is_success() {
        eprintln!("Server returned non-success status for seed bytes: {}", seed_response.status());
        return;
    }

    let seed_body_bytes = match seed_response.bytes() {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Error reading seed response body: {}", e);
            return;
        }
    };

    let mut seed_cursor = Cursor::new(seed_body_bytes);

    let seed_data_len = match seed_cursor.read_u32::<LittleEndian>() {
        Ok(len) => len as usize,
        Err(e) => {
            eprintln!("Error reading seed data length prefix: {}", e);
            return;
        }
    };

    println!("Reported seed data length: {} bytes", seed_data_len);

    if seed_cursor.get_ref().len() < 4 + seed_data_len {
        eprintln!("Error: Seed response body is shorter than reported length.");
        return;
    }

    let mut seed_bytes = vec![0u8; seed_data_len];
    match seed_cursor.read_exact(&mut seed_bytes) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error reading seed bytes: {}", e);
            return;
        }
    }


    println!("Successfully read {} bytes of seed data.", seed_bytes.len());
    println!("Raw Seed Bytes: {:?}", seed_bytes);

    match String::from_utf8(seed_bytes.clone()) {
        Ok(s) => println!("Seed as String: {}", s),
        Err(_) => println!("Seed bytes are not valid UTF-8."),
    }


    println!("\n--- Fetching and Parsing Iterations ---");

    let iterations_url = "http://127.0.0.1:8000/num_iterations";
    println!("Fetching iteration bytes from: {}", iterations_url);

    let iterations_response = match get(iterations_url) {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("Error fetching iteration bytes: {}", e);
            return;
        }
    };

    if !iterations_response.status().is_success() {
        eprintln!("Server returned non-success status for iteration bytes: {}", iterations_response.status());
        return;
    }

    let iterations_body_bytes = match iterations_response.bytes() {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Error reading iteration response body: {}", e);
            return;
        }
    };

    let mut iterations_cursor = Cursor::new(iterations_body_bytes);

    let iterations_data_len = match iterations_cursor.read_u32::<LittleEndian>() {
        Ok(len) => len as usize,
        Err(e) => {
            eprintln!("Error reading iteration data length prefix: {}", e);
            return;
        }
    };

    println!("Reported iteration data length: {} bytes", iterations_data_len);

    if iterations_data_len != 4 {
        eprintln!("Error: Expected iteration data length of 4 bytes, but got {}.", iterations_data_len);
        return;
    }

    let mut num_iterations_bytes: [u8; 4] = [0u8; 4];
    match iterations_cursor.read_exact(&mut num_iterations_bytes) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error reading iteration bytes: {}", e);
            return;
        }
    }

    let num_iterations = u32::from_le_bytes(num_iterations_bytes);

    println!("Successfully read {} bytes for iteration count.", num_iterations_bytes.len());
    println!("Number of Iterations: {}", num_iterations);

    println!("\n--- Client Finished ---");
}

