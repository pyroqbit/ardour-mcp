use nannou_osc;
// use nannou_osc::recv::DEFAULT_MTU; // Unused import, as _packet_buf is not used
use std::env;
use std::net::SocketAddr;

// It seems nannou_osc::Packet is an alias for rosc::OscPacket.
// For matching bundle contents, we might need to be explicit with the rosc path.
use nannou_osc::rosc; // Add this use statement

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <listen_address:port>", args[0]);
        eprintln!("Example: {} 127.0.0.1:9099", args[0]);
        std::process::exit(1);
    }

    let listen_addr_str = &args[1];
    let listen_addr: SocketAddr = listen_addr_str.parse().expect("Invalid listen address format");

    println!("Listening for OSC messages on port: {}", listen_addr.port());

    let receiver = nannou_osc::receiver(listen_addr.port())?;
    
    loop {
        match receiver.recv() {
            Ok((packet, src_addr)) => {
                match packet {
                    nannou_osc::Packet::Message(msg) => {
                        println!("OSC Message from {}: {} {:?}", src_addr, msg.addr, msg.args);
                    }
                    nannou_osc::Packet::Bundle(bundle) => {
                        println!("OSC Bundle from {}:", src_addr);
                        for p_in_bundle in bundle.content {
                            match p_in_bundle {
                                rosc::OscPacket::Message(m) => {
                                    println!("  - Bundle Message: {} {:?}", m.addr, m.args);
                                }
                                rosc::OscPacket::Bundle(_) => {
                                    println!("  - Nested Bundle (not decoded further)");
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error receiving OSC packet: {}. Attempting to continue.", e);
                // If recv is blocking, an error might be more critical.
                // For a simple monitor, we can log and try to continue, or exit.
                // Let's sleep briefly and continue to see if it recovers.
                std::thread::sleep(std::time::Duration::from_millis(500)); 
            }
        }
    }
} 