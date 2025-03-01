use std::{collections::HashMap, fmt, str::FromStr};

use bytes::Bytes;
use ed25519_dalek::Signature;
use eframe::egui;
use futures_lite::StreamExt;
use iroh::{Endpoint, NodeAddr, PublicKey, SecretKey, protocol::Router};
use iroh_blobs::net_protocol::Blobs;
use iroh_gossip::{
    net::{Gossip, GossipEvent, GossipReceiver},
    proto::TopicId,
};
use serde::{Deserialize, Serialize};

#[derive(Default)]
struct ThreeApp {}

impl ThreeApp {
    fn name() -> &'static str {
        "three"
    }
}

impl eframe::App for ThreeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.5);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("This is a ui.heading. ");

            ui.label("This is a ui.label");

            // This literally creates the button AND checks to see if it was clicked
            if ui.button("Quit").clicked() {
                std::process::exit(0);
            };
        });
    }
}

#[derive(Serialize, Deserialize)]
struct Ticket {
    topic: TopicId,
    peers: Vec<NodeAddr>,
}

impl Ticket {
    /// Deserializes from bytes.
    fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        postcard::from_bytes(bytes).map_err(Into::into)
    }
    /// Serializes to bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        postcard::to_stdvec(self).expect("postcard::to_stdvec is infallible")
    }
}

/// Serializes to base32.
impl fmt::Display for Ticket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut text = data_encoding::BASE32_NOPAD.encode(&self.to_bytes()[..]);
        text.make_ascii_lowercase();
        write!(f, "{}", text)
    }
}

/// Deserializes from base32.
impl FromStr for Ticket {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = data_encoding::BASE32_NOPAD.decode(s.to_ascii_uppercase().as_bytes())?;
        Self::from_bytes(&bytes)
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum Message {
    About { name: String },
    Post { text: String },
}

// #[tokio::main]
// async
fn main() -> anyhow::Result<()> {
    // let app_state = ThreeApp::new().await?;

    // let secret_key = SecretKey::generate(rand::rngs::OsRng);

    // let endpoint = Endpoint::builder()
    //     .secret_key(secret_key)
    //     .discovery_n0()
    //     .bind()
    //     .await?;
    // let blobs = Blobs::memory().build(&endpoint);
    // let gossip = Gossip::builder().spawn(endpoint.clone()).await?;
    // let router = Router::builder(endpoint.clone())
    //     .accept(iroh_blobs::ALPN, blobs.clone())
    //     .accept(iroh_gossip::ALPN, gossip.clone())
    //     .spawn()
    //     .await?;
    // let blobs_client = blobs.client();

    // let topic = TopicId::from_bytes(rand::random());
    // let ticket = {
    //     let me = endpoint.node_addr().await?;
    //     let peers = vec![].iter().cloned().chain([me]).collect();
    //     Ticket { topic, peers }
    // };
    // println!("> ticket to join us: {ticket}");

    // let node_id = router.endpoint().node_id();

    // // let peer_ids = peers.iter().map(|p| p.node_id).collect();
    // let (sender, receiver) = gossip.subscribe_and_join(topic, vec![]).await?.split();

    // tokio::task::spawn(subscribe_loop(receiver));

    // // spawn an input thread that reads stdin
    // // not using tokio here because they recommend this for "technical reasons"
    // let (line_tx, mut line_rx) = tokio::sync::mpsc::channel(1);
    // std::thread::spawn(move || input_loop(line_tx));

    // println!("> type a message and hit enter to broadcast...");
    // while let Some(text) = line_rx.recv().await {
    //     let message = Message::Post { text: text.clone() };
    //     let encoded_message = SignedMessage::sign_and_encode(endpoint.secret_key(), &message)?;
    //     sender.broadcast(encoded_message).await?;
    //     println!("> sent: {text}");
    // }

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((400.0, 400.0)),
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(
        ThreeApp::name(),
        native_options,
        Box::new(|_| Ok(Box::<ThreeApp>::default())),
    )
    .unwrap();

    // TODO app_state.router.shutdown().await?;

    Ok(())
}

async fn subscribe_loop(mut receiver: GossipReceiver) -> anyhow::Result<()> {
    // init a peerid -> name hashmap
    let mut names = HashMap::new();
    while let Some(event) = receiver.try_next().await? {
        if let iroh_gossip::net::Event::Gossip(GossipEvent::Received(msg)) = event {
            let (from, message) = SignedMessage::verify_and_decode(&msg.content)?;
            match message {
                Message::About { name } => {
                    names.insert(from, name.clone());
                }
                Message::Post { text } => {
                    let name = names
                        .get(&from)
                        .map_or_else(|| from.fmt_short(), String::to_string);
                    println!("{}: {}", name, text);
                }
            }
        }
    }
    Ok(())
}

fn input_loop(line_tx: tokio::sync::mpsc::Sender<String>) -> anyhow::Result<()> {
    let mut buffer = String::new();
    let stdin = std::io::stdin(); // We get `Stdin` here.
    loop {
        stdin.read_line(&mut buffer)?;
        line_tx.blocking_send(buffer.clone())?;
        buffer.clear();
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SignedMessage {
    from: PublicKey,
    data: Bytes,
    signature: Signature,
}

impl SignedMessage {
    pub fn verify_and_decode(bytes: &[u8]) -> anyhow::Result<(PublicKey, Message)> {
        let signed_message: Self = postcard::from_bytes(bytes)?;
        let key: PublicKey = signed_message.from;
        key.verify(&signed_message.data, &signed_message.signature)?;
        let message: Message = postcard::from_bytes(&signed_message.data)?;
        Ok((signed_message.from, message))
    }

    pub fn sign_and_encode(secret_key: &SecretKey, message: &Message) -> anyhow::Result<Bytes> {
        let data: Bytes = postcard::to_stdvec(&message)?.into();
        let signature = secret_key.sign(&data);
        let from: PublicKey = secret_key.public();
        let signed_message = Self {
            from,
            data,
            signature,
        };
        let encoded = postcard::to_stdvec(&signed_message)?;
        Ok(encoded.into())
    }
}
