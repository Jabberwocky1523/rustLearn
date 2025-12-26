
use std::{collections::HashMap, io, vec};

use crate::{ list::LinkList};
mod list;
/// 生成并打印前 n 个斐波那契数
fn fibonacci(n: u32) {
    let (mut current, mut next) = (1u64, 1u64);
    
    for _ in 0..n {
        println!("{}", current);
        (current, next) = (next, current + next);
    }
}
fn main() {
  let mut s1 = String::from("123");
  let s2 = s1.clone();
  println!("{} {}",s1,s2);
  let x :(i32,i32) = (1,2);
  let mut link = LinkList::new();
  let mut map:HashMap<String,i32> = HashMap::new();
  map.insert(String::from("123"), 123);
  let a = match map.get("123") {
      Some(state)=>*state,
      None=>2
  };
  print!("{}",a);
}
//   let mut buf = String::new();
    
//     io::stdin()
//         .read_line(&mut buf)
//         .expect("读取输入失败");
    
//     let n: u32 = buf.trim()
//         .parse()
//         .unwrap_or(0);
    
//     fibonacci(n);
// fn main() {
//     let mut list = LinkList::new();
//     list.push(1);
//     list.push(2);
//     list.print();
// }
// let s_n: i32 = rand::thread_rng().gen_range(1..101);
// let mut sum: i32 = 0;
// loop {
//     let mut number: String = String::new();
//     println!("Input number:");
//     io::stdin().read_line(&mut number).expect("error read");
//     let number: i32 = match number.trim().parse() {
//         Ok(num) => num,
//         Err(_) => {
//             println!("错误的类型");
//             break;
//         }
//     };
//     add(sum, 1);
//     match number.cmp(&s_n) {
//         Ordering::Equal => {
//             println!("Equal!");
//             break;
//         }
//         Ordering::Greater => {
//             println!("Greater!");
//         }
//         Ordering::Less => {
//             println!("Less!");
//         }
//     };
// }
// print!("{}", sum);
