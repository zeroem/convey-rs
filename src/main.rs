#[macro_use]
extern crate clap;

extern crate iron;

#[macro_use]
extern crate log;
extern crate env_logger;

mod conveyors;

fn main() {
    env_logger::init().unwrap();

    let matches = clap_app!(conveyors =>
        (version: "0.0.1")
        (author: "Darrell Hamilton <darrell.noice@gmail.com>")
        (about: "A dynamic TCP/HTTP reverse proxy")
        (@subcommand doc =>
            (about: "Retrieve documentation on conveyors")
        )
        (@subcommand start =>
            (about: "Start conveyors reverse proxy")
            (@arg ADMIN_ADDR: --admin +takes_value "Sets the bind address for the admin API")
        )
        (@subcommand admin =>
            (about: "Administer a running instance of conveyors")
        )
        (@subcommand metrics =>
            (about: "Query metrics from a running instance of conveyors")
        )
    ).get_matches();

    let mut conveyors = self::conveyors::Conveyors::new();

    if let Some(start_matches) = matches.subcommand_matches("start") {
        if let Some(admin_bind) = start_matches.value_of("ADMIN_ADDR") {
            conveyors.admin_bind(String::from(admin_bind));
        }

        conveyors.start()
    }
}
