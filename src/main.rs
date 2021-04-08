use std::net::TcpStream;
use std::env;

// Function to store open ports on target
static mut PORTS:Vec<u16> = vec![];

fn main(){

    let args: Vec<String> = env::args().collect();
    let address = &args[1];
    
    // Calling scanports function
    scanports(address);

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