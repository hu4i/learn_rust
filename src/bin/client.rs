use mini_redis::client;
use tokio::sync:: {
    mpsc,
    oneshot,
};
use learn_rust::my_redis::{
    Command::{self, *},
    client_dbg_print,
};

#[tokio::main]
async fn main() {

    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {

            match cmd {
                Get { key, resp } => {
                    let res = client.get(&key).await;
                    let _ = resp.send(res);
                }
                Set { key, val, resp } => {
                    let res = client.set(&key, val).await;

                    let _ = resp.send(res);
                }
            }
        }
    });

    
    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();

        let cmd = Command::Get {
            key: "hello".to_string(),
            resp: resp_tx,
        };

        tx.send(cmd).await.unwrap();

        let res = resp_rx.await;

        client_dbg_print(&format!("GOT = {:?}", res));

    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp: resp_tx,
        };

        tx2.send(cmd).await.unwrap();
        
        let res = resp_rx.await;

        client_dbg_print(&format!("GOT = {:?}", res));
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}