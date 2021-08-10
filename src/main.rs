use std::fmt::Debug;
use std::any::Any;
use std::collections::VecDeque;

use std::sync::{Arc, Mutex};
//use std::ptr::null;

use std::thread;
use thread::JoinHandle;

type ThreadFn = Arc<Mutex<FnMut(Arc<Mutex<Conn>>) -> bool + Sync + Send>>;

struct Process {
    pub exec: ThreadFn,
    pub connection: Arc<Mutex<Conn>>,
    pub closed: bool,
}


impl Process {
    pub fn new(connection: Arc<Mutex<Conn>>, exec: ThreadFn) -> Self {
        Process {
            exec,
            connection,
            closed: false,
        }
    }

    pub fn execute(self: &Self) -> JoinHandle<()> {
        let exec_cloned = self.exec.clone();
        let connection_cloned = self.connection.clone();
        thread::spawn(move || {
            for _ in 1..=10 {      // not real - I just want to show the exec being invoked more than once!
                (exec_cloned.lock().unwrap())(connection_cloned.clone());

            }
        })
    }
}

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


unsafe impl Sync for IP {}

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
        println!("Received: {:#?}", &val.data);
        self.conn.push_back(val);
        return true;

    }
}

unsafe impl Send for Conn {}

unsafe impl Sync for Conn {}


fn main() {
    let conn = Arc::new(Mutex::new(Conn::new(5)));


    // pub  fn mySender() {
    //     let val = IP::new(Box::new(String::from("hello")));
    //     conn.send(val);
    //  }


    let closure = |conn: Arc<Mutex<Conn>>| -> bool {
        let val = IP::new(Box::new(String::from("hello")));
        conn.lock().unwrap().send(val);
        true
    };

    let mut proc_a: Process = Process::new(conn.clone(),
                                           Arc::new(Mutex::new(closure)));

    let process_handle = proc_a.execute();
    process_handle.join();

}
