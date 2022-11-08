

fn main() {
    const NUM_MESSAGES:usize = 1000;
    let receiver = std::net::UdpSocket::bind("127.0.0.1:0").unwrap();
    let sender = std::net::UdpSocket::bind("127.0.0.1:0").unwrap();

    let sender_future = async{
        for _ in 0..NUM_MESSAGES {
            let _ = sender.send_to(b"Hola mundo", receiver.local_addr().unwrap());
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    };
    let receiver_future = async{
        let mut buffer = [0; 256];
        let mut count:i32 = 0;
        for _ in 0..NUM_MESSAGES {
            let _ = receiver.recv_from(&mut buffer).unwrap();
            count+=1;
            println!("Recibimos {} mensajes", count);
        }
    };

    futures::executor::block_on(async {
        futures::join!(sender_future, receiver_future);
    })


}
