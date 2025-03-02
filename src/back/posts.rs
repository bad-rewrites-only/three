use sorted_vec::SortedSet;
use std::{cmp::Ordering, collections::HashSet, time::SystemTime};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Posts(SortedSet<Post>);

impl PartialOrd for Post {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Post {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare by time, with the most recent post first
        self.time.cmp(&other.time).reverse()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Post {
    name: String,
    time: SystemTime,
    text: String,
    likes: HashSet<String>,
    like_count: u64,
    comments: Vec<Comment>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment {
    name: String,
    time: SystemTime,
    text: String,
    likes: HashSet<String>,
    like_count: u64,
}

impl Post {
    pub fn new(name: String, text: String) -> Post {
        Post {
            name,
            time: SystemTime::now(),
            text,
            likes: HashSet::new(),
            like_count: 0,
            comments: Vec::new(),
        }
    }
}

impl Comment {
    pub fn new(name: String, text: String) -> Comment {
        Comment {
            name,
            time: SystemTime::now(),
            text,
            likes: HashSet::new(),
            like_count: 0,
        }
    }
}
