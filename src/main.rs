use clap::{App, Arg, SubCommand};
use rand::Rng;

fn main() {
    let matches = App::new("raft-rs")
        .version("0.1")
        .author("simon jiao <simonjiaoh@gmail.com>")
        .about("generate uuid base on raft consensus")
        .subcommands(vec![
            SubCommand::with_name("start")
                .arg(
                    Arg::with_name("id")
                        .long("id")
                        .value_name("NODEID")
                        .help("an integer node id")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("raft_port")
                        .long("raft_port")
                        .value_name("RAFT_PORT")
                        .help("raft port")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("http_port")
                        .long("http_port")
                        .value_name("HTTP_PORT")
                        .help("http port")
                        .takes_value(true),
                ),
            SubCommand::with_name("join")
                .arg(Arg::with_name("id").value_name("NODE").takes_value(true)),
            SubCommand::with_name("leave"),
        ])
        .get_matches();

    match matches.subcommand_name() {
        Some("start") => {
            if let Some(start) = matches.subcommand_matches("start") {
                let id = match start.value_of("id") {
                    Some(id) => match id.parse::<usize>() {
                        Ok(id) => id,
                        Err(err) => {
                            println!("Error {}", err);
                            println!("{}", matches.usage());
                            return;
                        }
                    },
                    _ => rand::thread_rng().gen::<usize>(),
                };
                let raft_port = start.value_of("raft_port").unwrap_or("9001");
                let http_port = start.value_of("http_port").unwrap_or("8001");

                println!("ticketd start {}", id);
            } else {
                println!("Please specify arguments!");
                println!("{}", matches.usage());
                return;
            }
        }
        Some("join") => {
            println!("ticketd join");
            return;
        }
        Some("leave") => {
            println!("ticketd leave");
            return;
        }
        None => {
            println!("No subcommand used");
            return;
        }
        _ => {
            println!("Invalid subcommand");
            return;
        }
    }
}
