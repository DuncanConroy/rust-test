//use std::any::{Any, TypeId};
use std::any::{Any};
use std::thread;

fn main() {
    struct IP <'a>{
        data: &'a dyn Any,
        owner: &'a FBPComponent
    }

	impl IP {
        pub fn new(data : dyn Any) -> Self {
            let x: IP;
            let x.data = data : dyn Any;           
            let.x.owner = {unknown};
            x
        }
       
    }
   
    struct Conn <'a> {
        conn: [&'a IP<'a>],
        nextGet: u32,
        nextPut: u32,
        cap: u32
    }

    impl Conn <'_> {
        pub fn new(cap: u32) -> Self {
            let x: Conn;
            let x: [&IP; cap] = Default::default();            
            x.nextGet = 0;
            x.nextPut = 0;
            x.cap = cap;
            x
        }
        pub fn send(val : IP) -> bool {
           conn[nextPut] = val; 
           nextPut = nextPut + 1;
           if nextPut >= conn.len() {
              nextPut = 0;
			}
 	       true; 
        }
    }

    
    let mut conn = Conn::new(5);

    thread::spawn(move || {
        let mut val = IP::new("hello");       
        conn.send(val).unwrap();
    });

    //let received = rx.recv().unwrap();
    //println!("Got: {}", received);
}
