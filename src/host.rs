use std::net::UdpSocket;

pub mod capture;
pub mod capture_xcap;
pub fn start_host() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:6769")?;

        // -- test data receive + send ---
        // let mut buf = [0; 1024];
        // let (amt, src) = socket.recv_from(&mut buf)?;
        //
        // let buf = &mut buf[..amt];
        // buf.reverse();
        //
        // socket.send_to(buf, &src)?;
        // -------------------------------
    }
    Ok(())
}
