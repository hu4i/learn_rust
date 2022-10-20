use tokio::net::{ TcpListener, TcpStream };
use learn_rust::my_redis::{
    ShardedDb,
    get_db,
    server_dbg_print,
};
use mini_redis::{Connection, Frame};


#[tokio::main]
async fn main() {
    use learn_rust::my_redis::*;

    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    server_dbg_print("Listenning");

    let shared_db = new_shared_db(8);


    loop {
        let (socket, _) = listener.accept().await.unwrap();
        
        let shared_db = shared_db.clone();
        
        tokio::spawn(async move {
            process(socket, shared_db).await;
        });
    }
}


async fn process(socket: TcpStream, shared_db: ShardedDb) {
    use mini_redis::{Command::{self, Get, Set}};

    let mut connection = Connection::new(socket);
    
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = get_db(&shared_db, cmd.key())
                    .lock()
                    .unwrap();
                server_dbg_print(&format!("Set key:[{}]", cmd.key()));
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            },
            Get(cmd) => {
                let db = get_db(&shared_db, cmd.key()).lock().unwrap();
                server_dbg_print(&format!("Get key:[{}]", cmd.key()));
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            },
            cmd => panic!("Unimplemented {:?}", cmd),
        };
        connection.write_frame(&response).await.unwrap();
    }
}