pub mod back;
pub mod front;

use std::{collections::HashMap, fmt, str::FromStr};

use bytes::Bytes;
use ed25519_dalek::Signature;
use futures::{SinkExt, Stream};
use futures_lite::StreamExt;
use iced::{Subscription, Task, stream::try_channel};
use iroh::{Endpoint, NodeAddr, PublicKey, SecretKey, protocol::Router};
use iroh_blobs::net_protocol::Blobs;
use iroh_gossip::{
    net::{Event, Gossip, GossipEvent, GossipReceiver},
    proto::TopicId,
};
use serde::{Deserialize, Serialize};

use front::app::{self, Message, Screen};

pub struct Three {
    screen: app::Screen,
    name: String,
    secret_key: SecretKey,
    follows: Vec<Topic>,
    peers: Vec<NodeAddr>,
    my_posts: Vec<String>,

    router: Option<Router>,
}

#[derive(Serialize, Deserialize)]
pub struct Topic {
    topic_id: TopicId,

    #[serde(skip)]
    receiver: Option<GossipReceiver>,
}

impl Three {
    pub fn new() -> (Self, Task<Message>) {
        let secret_key = SecretKey::generate(rand::rngs::OsRng);
        let follows = vec![];
        let peers = vec![];
        let my_posts = vec![];
        let three = Self {
            name,
            secret_key: secret_key.clone(),
            follows,
            peers,
            my_posts,
            router: None,
        };
        (
            three,
            Task::done(Message::Init), // Task::perform(three.iroh_init(secret_key), Message::Refreshed),
        )
    }
    async fn iroh_init(secret_key: SecretKey) -> Router {
        let endpoint = Endpoint::builder()
            .secret_key(secret_key)
            .discovery_n0()
            .bind()
            .await
            .unwrap();
        let blobs = Blobs::memory().build(&endpoint);
        let gossip = Gossip::builder().spawn(endpoint.clone()).await.unwrap();
        let router = Router::builder(endpoint.clone())
            .accept(iroh_blobs::ALPN, blobs.clone())
            .accept(iroh_gossip::ALPN, gossip.clone())
            .spawn()
            .await
            .unwrap();
        // let blobs_client = blobs.client();

        let topic = TopicId::from_bytes(rand::random());
        let ticket = {
            let me = endpoint.node_addr().await.unwrap();
            let peers = vec![].iter().cloned().chain([me]).collect();
            Ticket { topic, peers }
        };
        println!("> ticket to join us: {ticket}");

        let node_id = router.endpoint().node_id();

        // router.shutdown().await.unwrap();
        // self.router = Some(router);

        router
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
struct SignedMessage {
    from: PublicKey,
    data: Bytes,
    signature: Signature,
}

impl SignedMessage {
    pub fn verify_and_decode(bytes: &[u8]) -> anyhow::Result<(PublicKey, Post)> {
        let signed_message: Self = postcard::from_bytes(bytes)?;
        let key: PublicKey = signed_message.from;
        key.verify(&signed_message.data, &signed_message.signature)?;
        let message: Post = postcard::from_bytes(&signed_message.data)?;
        Ok((signed_message.from, message))
    }

    pub fn sign_and_encode(secret_key: &SecretKey, message: &Post) -> anyhow::Result<Bytes> {
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

#[derive(Debug, Serialize, Deserialize)]
enum Post {
    About { name: String },
    Post { text: String },
}

async fn refresh_topics(
    receiver: &mut GossipReceiver,
    followed: &mut HashMap<PublicKey, String>,
    received: &mut HashMap<PublicKey, Vec<String>>,
) -> anyhow::Result<()> {
    while let Some(event) = receiver.try_next().await? {
        if let iroh_gossip::net::Event::Gossip(GossipEvent::Received(msg)) = event {
            let (from, message) = SignedMessage::verify_and_decode(&msg.content)?;
            match message {
                Post::About { name } => {
                    followed.insert(from, name);
                }
                Post::Post { text } => {
                    let history = received.get_mut(&from).unwrap();
                    let name = followed.get(&from).unwrap();
                    println!("{}: {}", name, text);
                    history.push(text);
                }
            }
        }
    }
    Ok(())
}

struct Feed {
    receiver: GossipReceiver,
    followed: HashMap<PublicKey, String>,
    received: HashMap<PublicKey, Vec<String>>,
}

enum FeedMessage {
    Refresh,
}

// impl Feed {
//     fn new() -> (Self, Task<Post>) {
//         (Self {}, Task::none())
//     }

//     fn update(&mut self, message: FeedMessage) -> Task<Post> {
//         match message {
//             FeedMessage::Refresh => Task::perform(refresh_topics(receiver, followed, received)),
//         }
//     }
// }
