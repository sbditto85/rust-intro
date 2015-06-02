#![feature(collections)]

extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;
extern crate awesome;

use iron::{Iron, IronResult, Protocol, Request, Response, status};
use router::{Router};
use mount::Mount;
use staticfile::Static;
use std::path::Path;
use awesome::{fizz_buzz, fib, fib_rec};

fn main() {
    //For routing dynamic handlers
    let mut router = Router::new();
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
        let n: Vec<u64> = (*query).replace("%20","").split(",").map(|n| n.trim().parse::<u64>().unwrap_or(default)).collect();
        let mut resp = String::from_str("Result: ");
        for (i, nstr) in  n.iter().map(|n| f(*n)).enumerate() {
            if i != 0 {
                resp.push_str(", ");
            }
            resp.push_str(&nstr);
        }
        resp
    } 

    //Send back the fizzbuzz of all the things
    fn fizz_buzz_handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
        let resp = handle_query(query, |n| fizz_buzz(n).to_string());
        Ok(Response::with((status::Ok, resp)))
    }
    
    //Send back the fib of all the things
    fn fib_handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
        let resp = handle_query(query, |n| fib(n).to_string());
        Ok(Response::with((status::Ok, resp)))
    }

    //Send back the fib of all the things
    fn fib_rec_handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
        let resp = handle_query(query, |n| fib_rec(n).to_string());
        Ok(Response::with((status::Ok, resp)))
    }
}
