pub mod queuelib {
    use std::ops::{Add, Index};

    pub struct queue {
        pub data: [i32; 5],
        pub font: usize,
        pub rear: usize,
    }
    impl queue {
        pub fn new() -> queue {
            return queue {
                data: [0; 5],
                font: 0,
                rear: 0,
            };
        }
        pub fn push(self, data: i32) {
            if ((self.rear + 1) % 5 == self.font) {
                panic!("queue stack!");
            }
            let tmp = self.rear + 1;
            self.rear = tmp;
        }
    }
}
