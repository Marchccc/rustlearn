use neon::prelude::*;
extern crate num_cpus;

// use std::thread;
// extern crate websocket;
// use websocket::sync::client::ClientBuilder;
// use websocket::sync::Server;
// use websocket::{OwnedMessage};
// const CONNECTION: &'static str = "ws://127.0.0.1:8080";
// const CONNECTION: &'static str = "ws://echo.websocket.org"; // 用公开服务器测试websocket代码是否可连通

// fn thread_count(mut cx: FunctionContext) -> JsResult<JsNumber> {
//     Ok(cx.number(num_cpus::get() as f64))
// }

// fn ele_testserver(mut cx: FunctionContext) -> JsResult<JsNumber> {
//     thread::spawn(|| {
//     let server = Server::bind("127.0.0.1:8080").unwrap();

// 	for request in server.filter_map(Result::ok) {
// 		// Spawn a new thread for each connection.
// 		thread::spawn(|| {
// 			if !request.protocols().contains(&"rust-websocket".to_string()) {
// 				request.reject().unwrap();
// 				return;
// 			}

// 			let mut client = request.use_protocol("rust-websocket").accept().unwrap();

// 			let ip = client.peer_addr().unwrap();

// 			println!("Connection from {}", ip);

// 			let message = OwnedMessage::Text("Hello".to_string());
// 			client.send_message(&message).unwrap();

// 			let (mut receiver, mut sender) = client.split().unwrap();

// 			for message in receiver.incoming_messages() {
// 				let message = message.unwrap();

// 				match message {
// 					OwnedMessage::Close(_) => {
// 						let message = OwnedMessage::Close(None);
// 						sender.send_message(&message).unwrap();
// 						println!("Client {} disconnected", ip);
// 						return;
// 					}
// 					OwnedMessage::Ping(ping) => {
// 						let message = OwnedMessage::Pong(ping);
// 						sender.send_message(&message).unwrap();
// 					}
// 					_ => sender.send_message(&message).unwrap(),
// 				}
// 			}
// 		});
//     }
// });
//     Ok(cx.number(num_cpus::get() as f64))
// }

// fn ele_test_ws(mut cx: FunctionContext) -> JsResult<JsNumber> {
//     println!("Connecting to {}", CONNECTION);

// 	let client = ClientBuilder::new(CONNECTION)
// 		.unwrap()
// 		.add_protocol("echo-protocol")
// 		.connect_insecure()
// 		.unwrap();

//     println!("Successfully connected");
//     let (mut receiver, mut sender) = client.split().unwrap();
//     sender.send_message(&OwnedMessage::Text("asdf".to_string()));
//     Ok(cx.number(num_cpus::get() as f64))
// }
extern crate reqwest;
use std::fs::*;
use std::io::{self,prelude::*,BufReader};
use std::time::Duration;
use std::sync::mpsc;
use std::string::String;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::env;

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

fn doit(mut cx: FunctionContext) -> JsResult<JsArray> {
    let arg0 = cx.argument::<JsString>(0)?.value();

    let mut res = env::current_exe().unwrap();
    res.pop();
    res.push("book");
    res.push("test.txt");
    let mut book = res.display();
    println!("config dir is : {}",res.display());
    let n_workers = 6; // 最大线程个数
    let pool = ThreadPool::new(n_workers); // 线程池
    // let mut handles = vec![]; // 存放线程
    let mut domain = String::from("."); // target
    domain = domain+&arg0;
    println!("{}", domain);
    let result = Arc::new(Mutex::new(vec![])); // 存放结果
    let file = File::open(res).expect("err"); // 字典
    let reader = BufReader::new(file);

    // 初始化http client
    let client = reqwest::Client::builder()
        .gzip(true)
        .timeout(Duration::from_secs(5)) // 每个请求10秒超时
        .build().expect("初始化http client失败");
    for line in reader.lines() {
        // 子进程不接受引用传值
        let c_domain = domain.clone();
        let c_client = client.clone();
        let v = result.clone();
        // 开线程
        pool.execute(move|| {
            let r = run(c_domain,c_client,line);
            // println!("run test result: {}", r);
            if r != "" {
                // println!("{}", r);
                let mut v = v.lock().unwrap();
                v.push(r);
            }
        });
        // println!("max count: {}", pool.active_count()); // 返回当前活动线程的数量。
        // println!("max count: {}", pool.max_count()); // 返回池将同时执行的最大线程数。
    }
    
    pool.join(); // 阻塞,等待线程池结束

    let v = result.clone();
    let v = v.lock().unwrap();
    // 迭代打印result
    // for elem in v.iter() {
    //     println!("{}", elem);
    // }
    
    // Create the JS array
    let js_array = JsArray::new(&mut cx, v.len() as u32);
    // Iterate over the rust Vec and map each value in the Vec to the JS array
    for (i, obj) in v.iter().enumerate() {
        let js_string = cx.string(obj);
        js_array.set(&mut cx, i as u32, js_string).unwrap();
    }
    Ok(js_array)
}

// 若HTTP状态码为200 OK，则return子域URL
fn run(domain: String,client: reqwest::Client,line: std::result::Result<String,std::io::Error>) -> String
{
    // 拼接子域URL
    let mut uri = String::from("");
    match line {
        Ok(ok) =>{
            uri = String::from("http://")+&ok+&domain;
        },
        Err(_) => println!("")
    }

    // 发送请求并校验HTTP状态码
    let res = client.get(&uri).send();
    if let Ok(ok) = res{
        if let reqwest::StatusCode::OK = ok.status() {
            return uri
        }
    }
    String::from("")
}
// 把thread_count函数导出到js函数,并命名为 threadCount
register_module!(mut m, {
    // m.export_function("threadCount", thread_count)?;
    // m.export_function("ele_test_ws", ele_test_ws)?;
    // m.export_function("ele_testserver", ele_testserver)?;
    m.export_function("doit", doit)?;
    Ok(())
});