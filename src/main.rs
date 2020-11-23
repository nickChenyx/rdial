//! 端口扫描工具
//! ulimit -n 限制会影响端口扫描的准确性，请确保扫描的端口范围大小在限制大小内。
//! 例如：connect 127.0.0.1:1959 err: Too many open files (os error 24)
//! @author: nickChenyx

#[macro_use]
extern crate may; // for macro go

use std::net::{ToSocketAddrs, Shutdown};
use may::net::TcpStream;
use std::time::Duration;
use crossbeam::sync::WaitGroup;
use std::sync::{Arc, Mutex};
use structopt::StructOpt;
use std::io;


#[derive(Debug, StructOpt)]
#[structopt(name="dial tool", about="A tool of server ports scanning")]
struct Opt {

    #[structopt(long="hostname", about="hostname to test", default_value="")]
    hostname: String,
    #[structopt(long="start-port", about="the port which scanning starts", default_value="80")]
    start_port: u16,
    #[structopt(long="end-port", about="the port which scanning ends", default_value="100")]
    end_port: u16,
    #[structopt(long="timeout", about="timeout for connect port", default_value="200")]
    timeout: u64,

}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let timeout = Duration::from_millis(opt.timeout);
    let vec: Vec<u16> = Vec::new();
    let arc_vec = Arc::new(Mutex::new(vec));

    // may::config().set_workers(20);
    // create a net wait group 
    let wg = WaitGroup::new();
    for n in opt.start_port..opt.end_port {
        // create another reference to the wait group
        let wg = wg.clone();
        let arc_vec = arc_vec.clone();
        let hostname = String::from(&opt.hostname);
        go!(move || {
            let flag = is_open(hostname, n, timeout);
            if flag {
                arc_vec.lock().unwrap().push(n);
            }
            drop(arc_vec);
            // drop the reference to the wait group
            drop(wg);
        });
    }
    // blocked until all coroutines finished
    wg.wait();
    println!("opened ports: {:?}", arc_vec.lock().unwrap());
}

fn is_open(hostname: String, port: u16, timeout: Duration) -> bool {
    // println!("server {} port {} scanning...", hostname, port);

    let server = format!("{}:{}", hostname, port);
    let addrs: Vec<_> = server.to_socket_addrs()
        .expect("unable to parse socket address")
        .collect();

    // 使用这样的判断的时候，coroutine 有问题，接口探测都是失败的。
    // 必须像下面一样，单独隔离出一条语句写 connect 逻辑，然后再去 if 判断结果才行
    // todo 这里为什么是这样呢
    // if let Ok(stream) = TcpStream::connect_timeout(&addrs[0], timeout) {
    let stream = TcpStream::connect_timeout(&addrs[0], timeout);

    if stream.is_ok() {
        // println!("server {} port {} open...", hostname, port);
        stream.unwrap().shutdown(Shutdown::Both).expect("shutdown tcp stream fail");
        return true;
    } else {
        // println!("server {} port {} close or timeout...", hostname, port);
        let err = stream.unwrap_err();
        match err.kind() {
            io::ErrorKind::ConnectionRefused => return false,
            io::ErrorKind::TimedOut => return false,
            _ => {
                println!("connect {} err: {}", &addrs[0], err);
                return false;
            },
        }
    }
}
