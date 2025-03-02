use three::Three;

use clap::Parser;
use iroh_gossip::proto::TopicId;

#[derive(Parser, Debug)]
struct Args {
    #[clap(long)]
    secret_key: Option<String>,
    #[clap(short, long)]
    name: Option<String>,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
enum Command {
    Run,
    Open { topic: Option<TopicId> },
    Join { ticket: String },
}

// #[tokio::main]
// async
fn main() -> anyhow::Result<()> {
    env_logger::init();
    // let mut args = Args::parse();
    // args.command = Command::Run;
    let comm = Command::Run;
    // match args.command {
    match comm {
        Command::Run => iced::application(Three::title, Three::update, Three::view)
            .run_with(Three::new)
            .unwrap(),
        Command::Open { topic } => todo!(),
        Command::Join { ticket } => todo!(),
        _ => {
            todo!()
        }
    };

    Ok(())
}
