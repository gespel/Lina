//use pcap::Device;
use std::net::TcpStream;
use std::io::Write;
mod tcpflooder;

fn main() {
    print_credits();
    //let mut cap = Device::lookup().unwrap().expect("ERROR").open().unwrap();

    /*while let Ok(packet) = cap.next_packet() {
        //println!("received packet! {:?}", packet);
        println!("{:?}", packet.to_ascii_lowercase());
    }*/
    let x = tcpflooder::TcpFlooder::new("127.0.0.1".to_string(), 9092);
    x.flood();
}

fn print_credits() {
    println!("Lina is written by Sten [G3spel] Heimbrodt!");
}