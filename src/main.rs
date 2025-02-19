use core::num;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
mod list;
use list::list::LinkList::LinkList;
fn main() {
    let mut list = LinkList::new();
    list.push(1);
    list.push(2);
    list.print();
}
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
