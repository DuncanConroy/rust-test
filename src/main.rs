use std::any::Any;
use std::collections::VecDeque;

use std::thread;


fn main() {

	struct ProcA<F>  
	where
  	  F: FnMut() -> bool,
	{
    	pub foo: F,
	}

    
	impl<F> ProcA<F>
	where
 	   F: FnMut() -> bool,
	{
   	 fn new(foo: F) -> Self {
       	 Self { foo }
    	}
	}


    #[derive(Debug)]
    struct Process {
	    exec: ProcA<F>,
        closed: bool,
		cnxt: Option<Conn> 
    }

	impl Process {
        pub fn  new(exec: new (foo< F>)) -> Self {
            Process {
                exec,
                closed: false,
                cnxt: None
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

    #[derive(Debug)]
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

	
    let foo = ProcA::<()> { foo: {
	    let val = IP::new(Box::new(String::from("hello")));
		let conn = self.cnxt;
        conn.send(val);
        return true; 
	} };
	
    

    let mut conn = Conn::new(5);

    let mut proc1 = Process::new(foo);
    proc1.cnxt = conn;

  // use Cacher as template for what I want to do!
    // https://doc.rust-lang.org/book/ch13-01-closures.html
    
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}
    
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

    thread::spawn(move || {proc1.exec}).join();


    
}
