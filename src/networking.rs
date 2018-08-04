
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate rmp_serialize;
extern crate rmp_serde as rmps;

//extern crate rmp_serialize;

#[derive(Serialize, Deserialize, Debug)]
enum Class {
    Mage,
    Warrior,
    Rogue,
    Druid
}

#[derive(Serialize, Deserialize, Debug)]
struct Player {
    name: String,
    position: Position,
    class: Class,
}

#[derive(Serialize, Deserialize, Debug)]
struct Position {
    x: i32,
    y: i32
}

use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    {
        let player = Player {
            name: "John Doe".to_string(),
            class: Class::Rogue,
            position: Position {
                x: 3, y: 4
            }
        };

        let player_bytes = rmps::to_vec(&Class::Rogue).unwrap();

        println!("player bytes: {:?}", String::from_utf8_lossy(&player_bytes));

        let serialized = serde_json::to_string(&player).unwrap();

        println!("player = {}", serialized);


        let socket = UdpSocket::bind("127.0.0.1:4313").unwrap();


        let mut buf = [0; 10];

        let (amt, src) = socket.recv_from(&mut buf)?;

        println!("Prislo mi `{}` bytu {}",
               String::from_utf8_lossy(&buf[..amt]),
               amt);

        // let buf = &mut buf[..amt];
        // buf.reverse();

        let mut kufr = [0u8; 64];

        let tuplik = (42u8, "kolecko", 0.3, "ge");

//        tuplik.encode(&mut Encoder::new(&mut &mut kufr[..])).unwrap();

        socket.send_to(&player_bytes, &src)?;

        // println!("Hello, world!");
    }

    Ok(())
}
