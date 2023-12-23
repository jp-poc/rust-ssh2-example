use std::io::prelude::*;
use std::env;
use dotenv::dotenv;
use std::net::TcpStream;
use ssh2::Session;
use std::str;

fn main() {
    dotenv().ok();
    let _device_hostname = env::var("DEVICE_HOSTNAME").expect("HOSTNAME is required.");
    let _device_username = env::var("DEVICE_USERNAME").expect("USERNAME is required.");
    let _device_password = env::var("DEVICE_PASSWORD").expect("PASSWORD is required.");
    
    let tcp = TcpStream::connect(format!("{}:22",_device_hostname)).unwrap();
    let mut session = Session::new().unwrap();
    session.set_tcp_stream(tcp);
    session.handshake().unwrap();

    session.userauth_password(&_device_username,&_device_password).unwrap();
    assert!(session.authenticated(),"Unable to authenticate.");

    let mut channel = session.channel_session().unwrap();
    channel.exec("show version").unwrap();

    let mut output = Vec::new();
    channel.read_to_end(&mut output).expect("Failed to read command output");
    println!("{:?}", str::from_utf8(&output))

}
