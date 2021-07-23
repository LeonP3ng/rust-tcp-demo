use std::net::TcpStream;
use std::io::prelude::*;
use std::str::from_utf8;

fn main() {
    //模式匹配，连接127.0.0.1的7878端口
    match TcpStream::connect("127.0.0.1:7878") {
        //连接成功
        Ok(mut stream) => {
            //输出信息
            println!("Successfully connected to server in port 7878");
            //发送消息msg
            let msg = b"Hello!";
            //将msg写入stream
            stream.write(msg).unwrap();
            //data作为接收server的消息
            let mut data = [0 as u8; 6];
            //读取server发来的数据，存入data中
            match stream.read_exact(&mut data) {
                //读取成功
                Ok(_) => {
                    //判断server发来的消息和msg一致
                    if &data == msg {
                        //输出信息
                        println!("Receive Server Reply!");
                    } else {
                        //转成string
                        let text = from_utf8(&data).unwrap();
                        //输出信息
                        println!("Unexpected reply: {}", text);
                    }
                },
                //读取失败
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        //连接失败
        Err(e) => {
            //输出信息
            println!("Failed to connect: {}", e);
        }
    }
}