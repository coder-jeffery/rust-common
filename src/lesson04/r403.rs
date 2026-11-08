
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
}
