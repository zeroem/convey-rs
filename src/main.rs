#[macro_use]
extern crate clap;

use admin;

fn main() {
    let matches = clap_app!(conveyors =>
        (version: "0.0.1")
        (author: "Darrell Hamilton <darrell.noice@gmail.com>")
        (about: "A dynamic TCP/HTTP reverse proxy")
        (@subcommand doc =>
            (about: "Retrieve documentation on conveyors")
        )
        (@subcommand start =>
            (about: "Start conveyors reverse proxy")
        )
        (@subcommand admin =>
            (about: "Administer a running instance of conveyors")
        )
        (@subcommand metrics =>
            (about: "Query metrics from a running instance of conveyors")
        )
    ).get_matches();


    println!("Hello, world!");
}
