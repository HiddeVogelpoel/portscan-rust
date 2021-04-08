use std::net::TcpStream;

use clap::{Arg, App};

// Function to store open ports on target
static mut PORTS:Vec<u16> = vec![];

fn main(){

    let matches = App::new("Portscanner")
        .version("0.0.3")
        .author("Hidde Vogelpoel")
        .about("Basic portscanner")
        .arg(Arg::with_name("target")
            .short("t")
            .long("target")
            .takes_value(true)
            .help("The target")
            .default_value("127.0.0.1"))
        .get_matches();
    
    // Calling scanports function 
    if let Some(t) = matches.value_of("target"){
        scanports(t);
    }

}

fn scanports(ip: &str){
    //Function is unsafe at the moment due to the PORTS variable
    unsafe{
        println!("Reached scanport with ip {}", ip);

        //TODO: add different portscan modes (all, top 1000) https://nullsec.us/top-1-000-tcp-and-udp-ports-nmap-default/
        //Loop to check all ports
        for i in 1..=65535{
        
            let ip = format!("{}:{}", ip, i);
        
            if let Ok(_stream) = TcpStream::connect(ip) {
                PORTS.push(i);
            }
        }

        for i in PORTS.iter(){
            println!("Port {} opened", i);
        }
    }
}