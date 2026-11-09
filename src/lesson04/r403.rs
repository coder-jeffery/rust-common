
//结构体 + Enum + Match

mod utils;
use rand::Rng;
use utils::hello;

enum Message{
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(u8, u8, u8),
}

struct User{
    id: u64,
    name: String,
}

impl User{
    fn new(id: u64, name: &str) -> Self{
        User{
            id,
            name: name.to_string(),
        }
    }

    fn send_msg(&self, msg: Message){
        println!("用户id:{}, 用户名称:{}, 发出消息", self.id,self.name);

        match msg{
            Message::Quit=> println!("退出"),
            Message::Move { x, y } => println!("移动 （{}, {}）", x, y),
            Message::Write(text)=> println!("文本：{}", text),
            Message::ChangeColor(r,g , b )=>{
                println!("修改颜色 ：{},{},{}", r,g,b);
            }

        }
    }
}


//枚举
// #[derive(Debug)]
// enum IpAddr{
//     V4(u8,u8,u8,u8),
//     V6(String),
// }


enum Status{
    Ok,
    Err(String),
}

impl Status{
    fn is_ok(&self) -> bool{
        match self {
            Status::Ok => true,
            Status::Err(_) => false,
        }
    }

    fn error_msg(&self)->Option<&str>{
        match self {
            Status::Err(msg) => Some(msg),
            Status::Ok => None,
        }
    }
}


fn main(){

    let user = User::new(1001, "jeff");

    
    user.send_msg(Message::Move { x: 10, y: 20 });
    user.send_msg(Message::Write("hello world".to_string()));
    user.send_msg(Message::ChangeColor(255, 0, 0));
    user.send_msg(Message::Quit);

    hello();

    //引入第三方库
    let n = rand::thread_rng().gen_range(1, 100);
    println!("random: {}", n);

    // let v4 = IpAddr::V4(127, 0, 0, 1);
    // let v6 = IpAddr::V6("::1".to_string());
    // println!("{:?}, {:?}", v4, v6);

    let res = Status::Ok;
    println!("{}", res.is_ok());

    let res2 = Status::Err("this is error message".to_string());
    if let Some(e) = res2.error_msg(){
        println!("error: {}", e);
    }


}
