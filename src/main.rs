use clap::Parser;
use iroh_gossip::proto::TopicId;
use three::Three;

#[derive(Parser, Debug)]
struct Args {
    #[clap(long)]
    secret_key: Option<String>,
    #[clap(short, long)]
    name: Option<String>,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser, Debug, Default)]
enum Command {
    #[default]
    Run,
    Open {
        topic: Option<TopicId>,
    },
    Join {
        ticket: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args = Args::parse();

    let three = Three::new()?;

    match args.command {
        Command::Run => {}
        // Command::Open { topic } => todo!(),
        // Command::Join { ticket } => todo!(),
        _ => {}
    }

    Ok(())
}
