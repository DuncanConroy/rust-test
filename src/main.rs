//use std::fmt::Debug;
use std::any::Any;
use std::collections::VecDeque;
use std::ptr::null;
use std::thread;


fn main() {

    #[derive(Debug)]
    struct FBPProcess {
        closed: bool,
    }

    #[derive(Debug)]
    struct IP {
        owner: Option<FBPProcess>,
        data: Box<dyn Any>,
    }

    impl IP {
        pub fn new(data: Box<dyn Any>) -> Self {
            IP {
                data,
                owner: None,
            }
        }
    }

    unsafe impl Send for IP {}

    struct Conn {
        cap: u32,
        conn: VecDeque<IP>,
    }

    impl Conn {
        pub fn new(cap: u32) -> Self {
            Conn {
                conn: VecDeque::new(),
                cap,
            }
        }

        pub fn send(self: &mut Self, val: IP) -> bool {
            println!("Received: {:#?}", &val);
            self.conn.push_back(val);
            return true;
        }
    }

    unsafe impl Send for Conn {}

    let mut conn = Conn::new(5);

    thread::spawn(move || {
        let val = IP::new(Box::new(String::from("hello")));
        conn.send(val);
    }).join();

    //let received = rx.recv().unwrap();
    //println!("Got: {}", received);
}
