use std::fmt::Debug;
use std::any::Any;
use std::collections::VecDeque;
use std::thread;
use thread::JoinHandle;


fn main() {

    //#[derive(Debug)]    
    struct Process {
        exec: Box<dyn FnMut()>,                             
        closed: bool 
    }

    impl Process {
        pub fn new() -> Self {
            Process {
                closed:false,
                exec:  Box<dyn FnMut()>,
            }

        }

        pub fn execute(self: &mut Self) -> JoinHandle<T> {
            thread::spawn(move || {
                for n in 1..=10 {      // not realistic - I just want to show the exec being invoked more than once!
                    (self.exec)();
                }
            });
        }
    }

    impl Default for Process {
      fn default() -> Self {
           Self::new()
      }
    }

    //#[derive(Debug)]
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
           // println!("Received: {:#?}", &val);
            self.conn.push_back(val);
            return true;

        }
    }

    unsafe impl Send for Conn {}

    let mut conn = Conn::new(5);

    
    //pub  fn mySender() {
    //    let val = IP::new(Box::new(String::from("hello")));  
    //    conn.send(val);
    // }

    let proc_a: Process = Process::new();

    proc_a.exec = Box<new|| -> bool {
        let val = IP::new(Box::new(String::from("hello")));       
        conn.send(val);    
    }>;

    proc_a.execute().join();
    
}
