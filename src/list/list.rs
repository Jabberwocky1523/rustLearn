use std::{boxed, option, os::unix::net::UnixDatagram, ptr::null, str::Lines};
struct LinkNode {
    data: i32,
    next: Option<Box<LinkNode>>,
}
pub struct LinkList {
    head: Option<Box<LinkNode>>,
}
impl LinkList {
    pub fn new() -> Self {
        return LinkList { head: None };
    }
    pub fn push(&mut self, data: i32) {
        let mut newnode = Box::new(LinkNode {
            data: data,
            next: self.head.take(),
        });
        self.head = Some(newnode);
    }

    pub fn print(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{}", node.data);
            current = &node.next;
        }
    }
}
