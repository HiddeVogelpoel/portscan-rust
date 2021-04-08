use std::net::TcpStream;
use std::env;

fn main(){

    let args: Vec<String> = env::args().collect();

    let address = &args[1];
    let mut ports = vec![];

    for i in 1..100{
        
        let ip = format!("{}:{}", address, i);
        
        if let Ok(_stream) = TcpStream::connect(ip) {
            ports.push(i);
        }
    }

    for i in ports.iter(){
        println!("Port {} opened", i);
    }
}
