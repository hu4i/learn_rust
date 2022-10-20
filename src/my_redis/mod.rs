use bytes::Bytes;
use tokio::sync::oneshot;
use std::{
    sync::{Arc, Mutex}, 
    collections::{HashMap, hash_map::DefaultHasher},
    hash::{Hash, Hasher},
};

pub type ShardedDb = Arc<Vec<Db>>;
pub type Db = Mutex<HashMap<String, Bytes>>;

pub fn new_shared_db(num_shareds: usize) -> ShardedDb {
    let mut db:Vec<Mutex<HashMap<_, _>>> = Vec::with_capacity(num_shareds);
    for _ in 0..num_shareds {
        db.push(Mutex::new(HashMap::new()));
    }
    Arc::new(db)
}

pub fn get_db<'a>(shared_db: &'a ShardedDb, key: &str) -> &'a Db {

    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    &shared_db[(hasher.finish() as usize) % shared_db.len() ]
}


pub fn server_dbg_print(description: &str) {
    println!("[*] [Server] {}", description);
}

pub fn client_dbg_print(description: &str) {
    println!("[*] [Client] {}", description);
}

#[derive(Debug)]
pub enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    }
}

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;