use std::cell::RefCell;
use std::rc::Rc;

struct User {
    username: String,
    age: u32,
    email: String,
}

struct Node<T> {
    value: T,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
}

impl User {
    fn sayhello(self) {
        println!(
            "hello my name is {}, i'm  {}, email is {}",
            self.username, self.age, self.email
        );
    }
}

fn largest_u32(a: u32, b: u32) -> u32 {
    if a > b { a } else { b }
}

fn largest_f32(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

// fn sayHello(user: &User){
//     println!("hello my name is {}, i'm  {}, email is {}", user.username, user.age, user.email)
// }

fn largest2<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn reverse_string(s: String) -> (String, String) {
    (s.clone(), s.chars().rev().collect())
}

fn append_string(s: &mut String) {
    s.push_str("!")
}

fn main() {
    let mut someon = User {
        username: String::from("jeff"),
        age: 35,
        email: String::from("19901712802@163.com"),
    };

    someon.age = 18;
    println!("username is {}", someon.username);
    println!("age is {}", someon.age);
    println!("email is {}", someon.email);

    let x = 1; //mut
    // x = 42;
    // let bool_flag =  false;
    println!("{} \n", x);
    // printf(bool_flag);

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{} \n", arr[1]);
    println!("Hello, world! this is first runst code!!! \n");

    // sayHello(&someon);
    someon.sayhello();

    println!("**************************** \n");

    println!("{}", largest_u32(1, 2));
    println!("{}", largest_f32(1.2, 2.2));
    // println!("{}", largest_f32(1,2));
    // println!("{}", largest2::<T: u32>(1,2));
    // println!("{}", largest2(1,2));
    println!("{}", largest2(1, 2));

    //rust 范性
    //Option<T>  Result<T>

    match std::env::home_dir() {
        Some(data) => println!("xx is some data= {:?}", data),
        None => println!("option is some"),
    }

    //Option<T> 代表有或者🈚无
    //Result<T,E> 代表成功或者失败

    // match std::env::var("LANG"){
    //     Ok(data) => println!("ok! {:?}", data),
    //     Err(err) => println!("err! {:?}", err),
    // }

    match std::env::var("NotExists") {
        Ok(data) => println!("ok! {:?}", data),
        Err(err) => println!("err! {:?}", err),
    }

    //Rust所有权 内存管理
    //变量的作用域
    //所有权

    let v1 = String::from("hhhhhh");
    println!("xxx {}", v1);
    {
        let v2 = String::from("neicunguanli");
        println!("xxx {}", v2);
    }
    // println!("xxx {}", v2);

    //作用域 所有权
    //引用和借用区别 （函数调用和传引用）

    let x = String::from("english name is jeffery");
    let (x_ret, y) = reverse_string(x);
    println!("{} {}", x_ret, y);

    let mut tmp = String::from("any string");
    append_string(&mut tmp);
    println!("{}", tmp);

    //二叉树  链表

    let mut root = Node::<u32> {
        value: 0,
        left: None,
        right: None,
    };
    let left = Node::<u32> {
        value: 1,
        left: None,
        right: None,
    };
    let right = Node::<u32> {
        value: 2,
        left: None,
        right: None,
    };

    root.left = Some(Rc::new(RefCell::new(left)));
    root.right = Some(Rc::new(RefCell::new(right)));

    println!("root = {:?}", root.value);
    // println!("left = {:?}", root.left.unwrap().value);
    // println!("right = {:?}", root.right.unwrap().value);

    if let Some(ref mut x) = root.left {
        x.borrow_mut().value = 4;
    }

    if let Some(ref x) = root.left {
        println!("left = {:?}", x.borrow().value);
    }
}

//test
