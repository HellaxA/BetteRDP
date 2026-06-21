use std::{net::UdpSocket, str::from_utf8};

pub fn start_client() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:0")?;
        socket.connect("127.0.0.1:6769")?;
        println!("connected to the host");

        // socket.send(b"Hello host !")?;
        //
        // let mut buf = [0; 1024];
        // let amt = socket.recv(&mut buf).unwrap();
        // let msg = from_utf8(&buf[..amt]).unwrap();
        //
        // println!("Received from host: {}", msg);
    }
    Ok(())
}
