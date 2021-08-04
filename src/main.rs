//use std::any::{Any, TypeId};
use std::any::{dyn Any};
use std::thread;
use std::ptr::null;

fn main() {
	
	struct FBPComponent{
        closed: bool
    }

	
    struct IP <'a>{
          owner: &'a FBPComponent,
		  data: &'a dyn Any 
    }

	impl IP <'_> {
        pub fn new(data : dyn Any) -> Self {
            let x: IP;
            x.data = data;           
            x.owner = null;
            x
        }
       
    }
   
    struct Conn <'a> {
      
        nextGet: usize,
        nextPut: usize,
        cap: u32,
 		//conn: [&'a IP<'a>] 
	    conn: Box<[&'a IP<'a>]> 
    }

    impl Conn <'_> {
        pub fn new(cap: u32) -> Self {
            let x: Conn;
            x.conn: [&IP; cap] = Default::default();            
            x.nextGet = 0;
            x.nextPut = 0;
            x.cap = cap;
            x
        }
    pub fn send(self: &Self, val : IP) -> bool {
           self.conn[self.nextPut] = val; 
           self.nextPut = self.nextPut + 1;
           if self.nextPut >= self.conn.len() {
              self.nextPut = 0;
			}
 	       return true; 
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
