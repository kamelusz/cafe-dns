use cafe_dns::QType;
use cafe_resolver::Resolver;

use std::process::exit;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt()]
pub struct Args {
    #[structopt(short, long)]
    host: String,

    #[structopt(short = "t", long, default_value = "A")]
    qtype: String,
}

fn main() {
    let args = Args::from_args();
    let qtype = match args.qtype.as_str() {
        "A" => QType::A,
        "SRV" => QType::SRV,
        _ => {
            eprintln!("Unsupported question type: {}", args.qtype);
            exit(1)
        }
    };

    let mut resolver = Resolver::new();
    let result = match qtype {
        QType::A => resolver.get_a_records(&args.host),
        QType::SRV => resolver.get_srv_records(&args.host),
    };

    match result {
        Err(err) => {
            println!("Error occured: {:?}", err);
            exit(1)
        }
        Ok(rs) => {
            for r in rs {
                println!("{}", r);
            }
        }
    }
}
