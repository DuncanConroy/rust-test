//use std::fmt::Debug;
use std::any::Any;
use std::collections::VecDeque;
//use std::ptr::null;
use std::thread;


fn main() {

    #[derive(Debug)]
    struct Process {
        exec: dyn Fn() -> (),
        closed: bool,
    }

    impl Process {
        pub fn new() -> Self {
            Process {
                exec,
                closed,
            }

        }
    }

    #[derive(Debug)]
    struct IP {
        owner: Option<Process>,
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

    
    pub fn mySender () {
        let val = IP::new(Box::new(String::from("hello")));
        conn.send(val);
    }

    let proc_a: Process = Process::new(mySender);

    thread::spawn(move || {
        for n in 1..=10 {
            proc_a.exec();
        }
    }).join();


    
}