//use std::fmt::Debug;
use std::any::Any;
use std::thread;
use std::ptr::null;
use std::collections::VecDeque;

fn main() {
	
	struct FBPComponent{
        closed: bool
    }

	
    struct IP <'a>{
          owner: &'a FBPComponent,
		  data: &'a dyn Any
    }

	impl IP <'_> {
        pub fn new(data : &dyn Any) -> Self {
            let x: IP;
            x.data = data;           
            x.owner = null;
            x
        }
       
    }
   
    struct Conn <'a>{
      
        cap: u32,
	   conn: VecDeque<IP<'a>>
    }

    impl Conn <'_> {
       pub fn new(cap: u32) -> Self {
            let x: Conn;
            x.conn: VecDeque<IP> = VecDeque::<IP>::new();       
            x.cap = cap;
            x
        }
    pub fn send(self: &Self, val : IP) -> bool {
           self.conn.push_back(val);
 	       return true; 
        }
    }

    
    let mut conn = Conn::new(5);

    thread::spawn(move || {
        let mut val = IP::new(&String::from("hello"));       
        conn.send(val);
    });

    //let received = rx.recv().unwrap();
    //println!("Got: {}", received);
}
