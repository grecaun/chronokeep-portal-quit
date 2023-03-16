use std::{net::{UdpSocket, Ipv4Addr, TcpStream}, str::FromStr, time::Duration, io::{Write, Read}};

pub const ZERO_CONF_REQUEST: &str = "[DISCOVER_CHRONO_SERVER_REQUEST]";
pub const ZERO_CONF_MULTICAST_ADDR: &str = "224.0.44.88";
pub const ZERO_CONF_PORT: u16 = 4488;
pub const ZERO_CONF_URI: &str = "224.0.44.88:4488";

pub const QUIT_REQUEST: &str = "{\"command\":\"quit\"}";

fn main() {
    println!("Attempting to shut down local portal.");
    let socket: UdpSocket = match UdpSocket::bind(String::from("0.0.0.0:0")) {
        Ok(sock) => sock,
        Err(e) => {
            println!("Error binding socket. {e}");
            return;
        }
    };
    println!("Setting timeout to 2 seconds.");
    match socket.set_read_timeout(Some(Duration::new(2, 0))) {
        Ok(_) => {},
        Err(e) => {
            println!("Unable to set read timeout on socket. {e}");
            return;
        }
    }
    println!("Joining multicast group.");
    match socket.join_multicast_v4(
        &Ipv4Addr::from_str(ZERO_CONF_MULTICAST_ADDR).unwrap(),
        &Ipv4Addr::UNSPECIFIED
    ) {
        Ok(_) => {},
        Err(e) => {
            println!("Error joining multicast group. {e}");
            return;
        }
    }
    println!("Requesting tcp port information.");
    match socket.send_to(ZERO_CONF_REQUEST.as_bytes(), ZERO_CONF_URI) {
        Ok(size) => {
            println!("Successfully wrote {size} bytes.");
        },
        Err(e) => {
            println!("Error sending discover message. {e}");
            return;
        }
    }
    let mut buf = [0; 1024];
    let port = match socket.recv_from(&mut buf) {
        Ok((num, src)) => {
            println!("Received {num} bytes from {src}.");
            let resp = match std::str::from_utf8(&buf[0..num]) {
                Ok(it) => it,
                Err(e) => {
                    println!("Error converting buffer to string. {e}");
                    return;
                }
            };
            let resp = resp.replace('[', "");
            let resp = resp.replace(']', "");
            let split: Vec<&str> = resp.split('|').collect();
            if split.len() < 3 {
                println!("Expected 3 results in message, found {}.", split.len());
                return;
            }
            let port: u16 = match split[2].parse() {
                Ok(p) => p,
                Err(e) => {
                    println!("Error parsing port. {e}");
                    return;
                }
            };
            port
        },
        Err(e) => {
            println!("Error trying to get reply from portal. {e}");
            return;
        }
    };
    println!("Connecting to tcp socket to send quit message.");
    let mut sock = match TcpStream::connect(format!("127.0.0.1:{}", port)) {
        Ok(s) => s,
        Err(e) => {
            println!("Error connecting to socket. {e}");
            return;
        }
    };
    println!("Sending quit command.");
    match sock.write(QUIT_REQUEST.as_bytes()) {
        Ok(num) => {
            println!("Successfully sent {num} bytes as shutdown command.");
        }
        Err(e) => {
            println!("Error writing quit command. {e}");
            return;
        }
    };
    println!("Waiting to see if the socket responds.");
    let mut recvd: String = Default::default();
    match sock.read_to_string(&mut recvd) {
        Ok(num) => {
            println!("Received {num} bytes.");
            println!("Message received from server: {recvd}");
        },
        Err(e) => {
            println!("Error receiving from the tcp socket. {e}");
        }
    }
    println!("All done. Goodbye.");
}
