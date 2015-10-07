//#![allow(dead_code)]
//#![allow(unused_imports)]

extern crate iron;
extern crate router;

extern crate staticfile;
extern crate mount;

use std::path::Path;
use std::process::Command;

use self::iron::prelude::*;
use self::iron::status;

use self::iron::{Url};
use self::router::{Router};

use self::staticfile::Static;
use self::mount::Mount;

//mod site;

pub fn build() -> Mount
{
    let mut m = Mount::new();
    let mut router = Router::new();

    router.get("/"   ,index);
    router.get("/:q" ,site_proc);
    router.get("/ls" ,ls);

    m.mount("/"   ,router);
    m.mount("/rs" ,Static::new(Path::new("hello.rs")));
    m.mount("/pic",Static::new(Path::new("pic.jpeg")));

    return m;
}

pub fn index(_: &mut Request) -> IronResult<Response>
{
    Ok(Response::with((status::Ok, "fn index -> Hello World!")))
}

pub fn ls(_: &mut Request) -> IronResult<Response>
{
    let output = Command::new("ls")
                     .output()
                     .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

    let out = String::from_utf8(output.stdout)
    //.unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
  .unwrap_or(String::new()); // "failed to from_utf8"

    Ok(Response::with((status::Ok, out)))
}

pub fn site_proc(req: &mut Request) -> IronResult<Response>
{
    let t_url = Url::parse("http://localhost:3000/test").unwrap();

    if req.url.path == t_url.path
    {
        Ok(Response::with((status::Ok, "test succ")))
    }
    else
    {
        Ok(Response::with((status::Ok, "def")))
    }
}
