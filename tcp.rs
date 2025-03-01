
/*
import necessary modules for the tcp server
*/
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};


fn handle_client(mut stream: TcpStream){
    //this is a buffer to read data from the client
    let mut buf = [0; 1024];
    
    //read the data from the stream and stores it in the buffer
    stream.read(&mut buf).expect("Failed to read from client");

    //convert the buffer to a UTF-8 encoded string
    let request = String::from_utf8_lossy(&buf[..]);

    println!("Request: {request}");

    let response = "Hello client!".as_bytes();

    //write the response to the stream
    stream.write(response).expect("Failed to write to client");

}

fn main() {
    //bind tcp listener to the address
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");

    println!("Server is listening on port 8080");

    for stream in listener.incoming(){
        //safer way of handling without any errors
        match stream{
            Ok(stream) => {
                std::thread::spawn(|| {handle_client(stream);});
            }
            Err(e) => {
                //stderr is used to print the error message
                eprintln!("Failed to establish a connection: {}", e);
            }

        }
    }
}
