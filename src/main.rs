extern crate iron;

mod site;
use site::*;

use iron::*;

fn main()
{
    println!("iron running on -> {0}",3000);

    Iron::new(build()).
          http("localhost:3000").
          unwrap();
}
