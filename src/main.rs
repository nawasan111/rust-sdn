use rust_ofp;
use rust_ofp::learning_switch::LearningSwitch;
use rust_ofp::ofp_controller::OfpController;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind(("127.0.0.1", 6653)).unwrap();
    for stream in listener.incoming() {
        println!("{:?}", stream);
        match stream {
            Ok(mut stream) => {
                println!("======================== stream ====================");
                std::thread::spawn(move || LearningSwitch::handle_client_connected(&mut stream));
            }
            Err(_) => {
                println!("!!! Found Error !!!");
                // panic!("Connection failed");
            }
        }
    }
}
