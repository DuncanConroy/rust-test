//use std::fmt::Debug;
use std::any::Any;
use std::collections::VecDeque;

use std::thread;


fn main() {

	struct Foo<F>
	where
  	  F: Fn() -> bool,
	{
    	pub foo: F,
	}

	impl<F> Foo<F>
	where
 	   F: Fn() -> bool,
	{
   	 fn new(foo: F) -> Self {
       	 Self { foo }
    	}
	}


    #[derive(Debug)]
    struct Process {
	    exec: Foo<F>,
        closed: bool,
		conn: Connection 
    }

	impl Process {
        pub fn new(exec: Foo<F>) -> Self {
            Process {
                exec,
                closed: false
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

	
    let foo = Foo { foo: {
	    let val = IP::new(Box::new(String::from("hello")));
		let conn = self.conn;
        conn.send(val);
        //return true; 
	} };
	
    

    let mut conn = Conn::new(5);

	let mut proc1 = Process::new(foo);
	proc1.cnxt = conn;


    thread::spawn(move || {proc1.exec}).join();


    
}
