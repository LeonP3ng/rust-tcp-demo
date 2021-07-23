use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    //调用bind函数绑定端口，返回一个TcpListener实例，ip地址为127.0.0.1，端口号为7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //输出信息
    println!("Server listening on port 7878");
    //一个stream代表client和server的一次连接
    for stream in listener.incoming() {
        //模式匹配
        match stream {
            //匹配成功
            Ok(stream) => {
                //调用handle_connection方法
                handle_connection(stream)      
            }
            //错误处理
            Err(e) => {
                //输出信息
                println!("Error: {}", e);
            }
        }
    }
    //关闭listener实例
    drop(listener);
}

fn handle_connection(mut stream: TcpStream){
    //分配一个缓冲区
    let mut buffer = [0 as u8; 6];
    //从stream读取数据到buffer
    stream.read(&mut buffer).unwrap();
    //将buffer转成string
    let buf  = String::from_utf8_lossy(&buffer[..]);
    //输出client发来的信息
    println!("Request: {}", buf.as_ref());
    //模式匹配
    match buf.as_ref() {
        //如果发来hello，对client回复
        "Hello!" => {
            println!("Receive Hello\n");
            //给client发送消息
            stream.write(&buffer[0..6]).unwrap();
        },
        //默认匹配，如果不符合上面的匹配规则，执行这句
        _ => println!("Receive something else!"),
    }
   
}


