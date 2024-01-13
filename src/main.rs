
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use rmodbus::{client::ModbusRequest, guess_response_frame_len, ModbusProto};
#[derive(Debug)]
enum Error {
    SomeError,
}
fn main() {
    let timeout = Duration::from_secs(1);

    // open TCP connection
    let  stream = TcpStream::connect("192.168.1.10:502").unwrap();
    stream.set_read_timeout(Some(timeout)).unwrap();
    stream.set_write_timeout(Some(timeout)).unwrap();

    // create request object
    for x in 1..254{
    let scan = || -> Result<(), Error>
        {
            read_reg(&stream, x);
            Ok(())
        };
        if let Err(_err) = scan(){
            println!("Cant Connect at {}",x);
        }

    }
}

fn read_reg(mut tcp_stream: &TcpStream, unit_id: u8) {
    let mut mreq = ModbusRequest::new(unit_id, ModbusProto::TcpUdp);

    // set 2 coils
    let mut request = Vec::new();

    // get  values back
    mreq.generate_get_inputs(0, 2, &mut request).unwrap();
    tcp_stream.write(&request).unwrap();
    let mut buf = [0u8; 6];
    tcp_stream.read_exact(&mut buf).unwrap();
    let mut response = Vec::new();
    response.extend_from_slice(&buf);
    let len = guess_response_frame_len(&buf, ModbusProto::TcpUdp).unwrap();
    if len > 6 {
        let mut rest = vec![0u8; (len - 6) as usize];
        tcp_stream.read_exact(&mut rest).unwrap();
        response.extend(rest);
    }
    let mut data = Vec::new();
    // check if frame has no Modbus error inside and parse response bools into data vec
    mreq.parse_u16(&response, &mut data).unwrap();
    for i in 0..data.len() {
        println!("Reading Values at Unit ID {}",unit_id);
        println!("{:?}",data);
    }
}