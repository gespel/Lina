use pcap::Device;

fn main() {
    let mut cap = Device::lookup().unwrap().expect("ERROR").open().unwrap();

    while let Ok(packet) = cap.next_packet() {
        //println!("received packet! {:?}", packet);
        println!("{:?}", packet.to_ascii_lowercase());
    }
}