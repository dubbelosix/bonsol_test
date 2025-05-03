use rouille;
use std::io::Cursor;
use byteorder::{LittleEndian, WriteBytesExt};
use bonsol_test_data::{SEED_MESSAGE, NUM_ITERATIONS};

fn main() {

    println!("Server started on http://127.0.0.1:8000");
    rouille::start_server("127.0.0.1:8000", move |request| {
        match request.url().as_str() {

            "/seed_bytes" => {
                println!("Received request for /seed_bytes. Responding with length-prefixed seed bytes.");

                let seed_bytes = SEED_MESSAGE.as_bytes();
                let data_len = seed_bytes.len();
                let mut buffer = Vec::with_capacity(4 + data_len);
                let mut cursor = Cursor::new(&mut buffer);
                cursor.write_u32::<LittleEndian>(data_len as u32).unwrap();

                buffer.extend_from_slice(seed_bytes);

                rouille::Response::from_data("application/octet-stream", buffer)
                    .with_unique_header("Connection", "close")
            }

            "/num_iterations" => {
                println!("Received request for /num_iterations. Responding with length-prefixed iteration count bytes.");

                let num_iterations_data = NUM_ITERATIONS.to_le_bytes();
                let data_len = num_iterations_data.len();

                let mut buffer = Vec::with_capacity(4 + data_len);

                let mut cursor = Cursor::new(&mut buffer);
                cursor.write_u32::<LittleEndian>(data_len as u32).unwrap();

                buffer.extend_from_slice(&num_iterations_data);

                rouille::Response::from_data("application/octet-stream", buffer)
                    .with_unique_header("Connection", "close")
            }

            _ => {
                println!("Received request for {}. Responding with 404.", request.url());
                rouille::Response::empty_404()
            }
        }
    });
}