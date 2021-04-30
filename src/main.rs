use std::str;
use std::string::String;

use std::net::UdpSocket;
use std::collections::HashMap;

use std::time::SystemTime;

fn main() 
{
	let mut map = HashMap::<String, SystemTime>::new();

	let port = 8086;
	println!("Starting daemon on port {}", port);

	let full_addr = format!("0.0.0.0:{}", port);
	let socket = UdpSocket::bind(full_addr).expect("Address binding failed");

	loop
	{
		let mut buf = [0; 1024];
		let (amt, src) = socket.recv_from(&mut buf).expect("Error handling data");
		let buf = &buf[..amt];
		let s = match str::from_utf8(buf)
		{
			Ok(v) => v,
			Err(_e) => {""}
		};

		if s.trim() == "status".trim()
		{
			let cur_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("Current time unknown?");
			let mut msg = format!("current time: {0}\n", cur_time.as_secs()).to_owned();

			for (k,v) in &map 
			{
				let time = v.duration_since(SystemTime::UNIX_EPOCH).expect("Invalid time?");

				let content = format!("server: {0}, time {1}\n", k, time.as_secs());
				msg.push_str(&content);
			}
			socket.send_to(msg.as_bytes(), src).expect("Cannot send back!");
		}
		else
		{
			socket.send_to("ok\n".as_bytes(), &src).expect("Cannot send back!");
			map.insert(src.ip().to_string(), SystemTime::now());
		}
	}
}
