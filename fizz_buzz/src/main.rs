#![feature(collections)]

extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;
extern crate awesome;


use std::path::Path;
use std::thread;
use std::sync::{Arc, mpsc, Mutex};

use iron::{Iron, IronResult, Protocol, Request, Response, status};
use router::{Router};
use mount::Mount;
use staticfile::Static;

use awesome::{fizz_buzz, fizz_buzz_enum, fib, fib_rec};

fn main() {
    //For routing dynamic handlers
    let mut router = Router::new();
    router.get("/fizzbuzz_enum/:query", fizz_buzz_enum_handler);
    router.get("/fizzbuzz/:query", fizz_buzz_handler);
    router.get("/fib/:query", fib_handler);
    router.get("/fib_rec/:query", fib_rec_handler);

    //For serving static files
    let mut mount = Mount::new();
    mount.mount("/api/", router);
    mount.mount("/", Static::new(Path::new("frontend")));

    //Fire off the server
    println!("Running on port 3000 ... ");
    Iron::new(mount).listen_with("localhost:3000", 100, Protocol::Http).unwrap();

    fn handle_query<F>(query: &str, f: F) -> String where F: Fn(u64) -> String {
        let default = 0;
        let ns: Vec<u64> = (*query).replace("%20","").split(",").map(|n| n.trim().parse::<u64>().unwrap_or(default)).collect();
        let mut resp = String::from_str("Result: ");
        for (i, nstr) in  ns.iter().map(|n| f(*n)).enumerate() {
            if i != 0 {
                resp.push_str(", ");
            }
            resp.push_str(&nstr);
        }
        resp
    }

    //Handle query with THREADS! and CHANNELS!
    fn handle_query_threaded<F>(query: &str, f: F) -> String where F: Fn(u64) -> String + Send + 'static {
        let default = 0;
        let ns: Vec<u64> = (*query).replace("%20","").split(",").map(|n| n.trim().parse::<u64>().unwrap_or(default)).collect();

        let (tx, rx) = mpsc::channel();

        let f_arc = Arc::new(Mutex::new(f));
        
        for (i, n) in  ns.iter().enumerate() {
            //Maybe a little smelly? not sure how else to do this.
            let tx = tx.clone();
            let f = f_arc.clone();
            let i_clone = i.clone();
            let n_clone = n.clone();
            thread::spawn(move || {
                let answer = f.lock().unwrap()(n_clone);

                if i_clone == 0 {
                    tx.send(answer).unwrap();
                } else {
                    tx.send(", ".to_string() + &answer).unwrap();
                }
                
            });
        }

        let mut resp = String::from_str("Result: ");
        for _ in ns {
            match rx.recv() {
                Ok(str) => resp.push_str(&str),
                Err(_) => resp.push_str("ERROR"),
            }
        }
        resp
    }

    //Send back the fizzbuzz_enum of all the things
    fn fizz_buzz_enum_handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("0");
        let resp = handle_query(query, |n| fizz_buzz_enum(n).to_string());
        Ok(Response::with((status::Ok, resp)))
    }
    
    //Send back the fizzbuzz of all the things
    fn fizz_buzz_handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("0");
        let resp = handle_query(query, |n| fizz_buzz(n).to_string());
        Ok(Response::with((status::Ok, resp)))
    }
    
    //Send back the fib of all the things
    fn fib_handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("0");
        let resp = handle_query_threaded(query, |n| fib(n).to_string());
        Ok(Response::with((status::Ok, resp)))
    }

    //Send back the fib of all the things
    fn fib_rec_handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("0");
        let resp = handle_query_threaded(query, |n| fib_rec(n).to_string());
        Ok(Response::with((status::Ok, resp)))
    }
}
