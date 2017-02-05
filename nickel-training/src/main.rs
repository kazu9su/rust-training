#[macro_use]
extern crate nickel;

use std::collections::HashMap;
use nickel::{Nickel, HttpRouter};

fn main() {
    let mut server = Nickel::new();

    server.get("/template", middleware! {|_req, _res|
        let mut data = HashMap::new();
        data.insert("color", "Green");
        data.insert("name", "Carifornia Apple");
       data.insert("price", "2.50");
       return _res.render("assets/hello.tpl", &data);
    });

    server.listen("192.168.33.10:2000");
}
