

/*

字符串
循环
结构体
Result

所有权：
    借用，生命周期

小项目：
 cli工具
 http服务器
 简单算法 数据结构


 标准库：
    Vec，HashMap，Iterator，Future
    unsafe，no_std, 异步

    Rust/ 报错分析：rust-analyzer + clippy

*/

#[derive(Debug)]
enum Status {
    Ok,
    Err(String),
}

impl Status {
    fn is_ok(&self) -> bool {
        match self {
            Status::Ok => true,
            Status::Err(_) => false,
        }
    }

    // 获取错误信息，Option
    fn err_msg(&self) -> Option<&str> {
        match self {
            Status::Err(msg) => Some(msg),
            Status::Ok => None,
        }
    }
}


fn main(){

    println!("this is main method!!!!!!");

    let s1 = Status::Ok;
    let s2 = Status::Err("something wrong".to_string());

    println!("s1 is_ok: {}", s1.is_ok());
    println!("s2 is_ok: {}", s2.is_ok());

    // 输出错误
    if let Some(e) = s2.err_msg() {
        println!("error: {}", e);
    }

    let r: Result<(), String> = Err("oops".to_string());
    if let Err(e) = r {
        println!("error: {e}");
    }
}