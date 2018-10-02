#![feature(plugin)]
#![plugin(tarpc_plugins)]

#[macro_use]
extern crate tarpc;
extern crate futures;
extern crate tokio_core;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use futures::Future;
use tarpc::future::client;
use tarpc::future::client::ClientExt;
use tarpc::util::FirstSocketAddr;
use tokio_core::reactor;

use std::io;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub description: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub id: i32,
    pub username: String,
}

service! {
    rpc get_user(id: i32) -> Option<User>;
    rpc insert_user(user: NewUser) -> Option<User>;
}

#[get("/<id>")]
fn index(id: String) -> String {
    // Send
            let options = client::Options::default().handle(reactor.handle());
            reactor
                .run(
                    FutureClient::connect("localhost:10000".first_socket_addr(), options)
                        .map_err(tarpc::Error::from)
                        .and_then(|client| client.get_user(id))
                        .map(|user| match user {
                            Some(value) => println!("The server responded with: {:#?}", value),
                            None => println!("The server responded with: No user"),
                        }),
                ).unwrap();
                "Hello"
}

fn main() {
    let mut reactor = reactor::Core::new().unwrap();

    loop {
        // Read command
        println!("Get: g");
        println!("Insert: i");
        let mut cmd = String::new();
        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read line");
        cmd = cmd.trim().to_string();

        if cmd == "g" {
            // Get
// Configuring rocket:
    let config = Config::build(Environment::Staging)
        .address("localhost")
        .port(9234)
        .finalize()
        .expect("failed to instantiate config");

    info!("igniting rocket");
    rocket::custom(config, false)
        .attach(logging::RocketLogger)
        .attach(banned::BanIpAddrs::default())
        .mount(
            "/",
            routes![index],
        ).launch();

            
}
