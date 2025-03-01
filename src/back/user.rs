use image::{ImageBuffer, Rgba};

struct User {
    public_key: String,
    name: Option<String>,
    posts: Option<Vec<Post>>,
    connections: Option<Vec<Connection>>,
}

struct Post {
    time: std::time::SystemTime,
    description: Option<String>,
    images: Option<Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>>,
    likes: u64,
}

struct Connection {
    user: User,
    depth: u64,
}
