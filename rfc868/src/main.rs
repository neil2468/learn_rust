use std::convert::TryInto;


fn main() {
    println!("Started");

    let socket = std::net::UdpSocket::bind("0.0.0.0:60000").unwrap();

    socket.send_to(&[0u8; 0], "time.nist.gov:37").unwrap();

    socket.set_read_timeout(Some(std::time::Duration::from_secs(5))).unwrap();
    let mut buf = [0u8; 32];
    let (amt, src) = socket.recv_from(&mut buf).unwrap();
    dbg!(amt, src, buf);

    //let x: &[u8; 4] = buf[0..4].try_into().unwrap();

    let (x, _) = buf.split_at(std::mem::size_of::<u32>());

    let time_now = u32::from_be_bytes(x.try_into().unwrap());
    dbg!(time_now);
    dbg!(amt, src, buf);

    println!("Finished");
}