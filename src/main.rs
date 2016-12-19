extern crate clap;
use clap::{App, Arg, SubCommand };

extern crate iron;

#[macro_use]
extern crate log;
extern crate env_logger;

mod conveyors;

use conveyors::frontend::Frontend;

fn main() {
    env_logger::init().unwrap();

    let matches =
        App::new("conveyors")
            .version("0.0.1")
            .author("Darrell Hamilton <darrell.noice@gmail.com>")
            .about("A dynamic TCP/HTTP reverse proxy")
            .subcommand(SubCommand::with_name("doc")
                            .about("Retrieve documentation on conveyors"))
            .subcommand(SubCommand::with_name("start")
                            .about("Start conveyors reverse proxy")
                            .arg(Arg::with_name("ADMIN_ADDR")
                                    .long("--admin")
                                    .takes_value(true)
                                    .help("Sets the bind address for the admin API"))
                            .arg(Arg::with_name("BIND")
                                    .long("--bind")
                                    .takes_value(true)
                                    .help("The address used to proxy requests")))
            .subcommand(SubCommand::with_name("admin")
                            .about("Administer a running instance of conveyors"))
            .subcommand(SubCommand::with_name("metrics")
                            .about("Query metrics from a running instance of conveyors"))
            .get_matches();

    let mut conveyors = self::conveyors::Conveyors::new();

    if let Some(start_matches) = matches.subcommand_matches("start") {
        if let Some(admin_bind) = start_matches.value_of("ADMIN_ADDR") {
            conveyors.admin_bind(String::from(admin_bind));
        }

        if let Some(bind) = start_matches.value_of("BIND") {
            conveyors.add_tcp_frontend(Frontend::new(bind));
        }

        conveyors.start()
    }
}
