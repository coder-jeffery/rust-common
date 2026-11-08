



/*

Rust所有权 OwnerShip

1. 每个值**只能有一个所有者**
2. 所有者离开作用域，值自动释放
3. 值不能同时有多个所有者
*/



/*
- **同一时间，可以有多个不可变引用 `&T`**
- **同一时间，最多只能有一个可变引用 `&mut T`**
- **不能同时存在可变引用 + 不可变引用**
*/

fn print(s: &String){
    println!("{}", s);
}




// fn print_slice<T: std::fmt::Debug>(s: &[T]){
//     println!("{:?}", s);
// }



/*
`arr[1..3]`：左闭右开，索引 `1、2`，所以是 `2, 3`
`&[T]` 就是切片类型
*/

fn main(){

    println!("hello world");

    let  s1 = String::from("hi! i'm jeffery");
    let s2 = s1;
    println!("{}", s2); //打印s1报错

    print(&s2);

    //  切片
    let arr = [1,2,3,4,5];
    let slice = &arr[1..3];

    println!("{:?}", slice);

    //遍历打印
    for v in slice{
        println!("{}", v);
    }
}