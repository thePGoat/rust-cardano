#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate iron;
extern crate router;

extern crate storage;
extern crate wallet_crypto;
extern crate blockchain;
extern crate exe_common;

use std::sync::{Arc};

use iron::Iron;

mod config;
mod handlers;

use config::{Config};

fn main() {
    use clap::{App, Arg, SubCommand};

    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("init")
                .about("init hermes environment")
        )
        .subcommand(
            SubCommand::with_name("start")
                .about("start explorer server")
        )
        .get_matches();

    let mut cfg = Config::open().unwrap_or(Config::default());

    match matches.subcommand() {
        ("init", _) => { cfg.save().unwrap(); },
        ("start", _) => {
            info!("Starting {}-{}", crate_name!(), crate_version!());
            let mut router = router::Router::new();
            let networks = Arc::new(cfg.get_networks().unwrap());
            handlers::block::Handler::new(networks.clone()).route(&mut router);
            handlers::pack::Handler::new(networks.clone()).route(&mut router);
            handlers::epoch::Handler::new(networks.clone()).route(&mut router);
            info!("listenting to port {}", cfg.port);
            Iron::new(router).http(format!("localhost:{}", cfg.port)).unwrap();
        },
        _ => {
            println!("{}", matches.usage());
            ::std::process::exit(1);
        },
    }
}
