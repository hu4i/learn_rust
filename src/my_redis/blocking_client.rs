//use std::simd::intrinsics;

use tokio::net::ToSocketAddrs;
use tokio::runtime::Runtime;
use bytes::Bytes;
use std::time::Duration;

pub use mini_redis::client::Message;


pub struct BlockingClient {
    inner: mini_redis::client::Client,
    rt: Runtime,
}

pub fn connect<T: ToSocketAddrs>(addr: T) -> mini_redis::Result<BlockingClient> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    Ok(BlockingClient {
        inner: rt.block_on(mini_redis::client::connect(addr))?,
        rt,
    })
}

impl BlockingClient {
    pub fn get(&mut self, key: &str) -> mini_redis::Result<Option<Bytes>> {
        self.rt.block_on(self.inner.get(key))
    }

    pub fn set(&mut self, key: &str, value: Bytes) -> mini_redis::Result<()> {
        self.rt.block_on(self.inner.set(key, value))
    }

    pub fn set_expires(
        &mut self,
        key: &str,
        value: Bytes,
        expiration: Duration,
    ) -> mini_redis::Result<()> {
        self.rt.block_on(self.inner.set_expires(key, value, expiration))
    }

    pub fn publish(&mut self, channel: &str, message:Bytes) -> mini_redis::Result<u64> {
        self.rt.block_on(self.inner.publish(channel, message))
    }
}

pub struct BlockingSubscriber {
    inner: mini_redis::client::Subscriber,
    rt: Runtime,
}

impl BlockingClient {
    pub fn subcribe(self, channels: Vec<String>) -> mini_redis::Result<BlockingSubscriber> {
        let subscriber = self.rt.block_on(self.inner.subscribe(channels))?;
        Ok(BlockingSubscriber { inner: subscriber, rt: self.rt })
    }
}

impl BlockingSubscriber {
    pub fn get_subscribed(&self) -> &[String] {
        self.inner.get_subscribed()
    }

    pub fn next_message(&mut self) -> mini_redis::Result<Option<Message>> {
        self.rt.block_on(self.inner.next_message())
    }

    pub fn subscribe(&mut self, channels: &[String]) -> mini_redis::Result<()> {
        self.rt.block_on(self.inner.subscribe(channels))
    }

    pub fn unsubscribe(&mut self, channels: &[String]) -> mini_redis::Result<()> {
        self.rt.block_on(self.inner.unsubscribe(channels))
    }
}