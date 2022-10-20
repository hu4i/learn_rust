use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

use crate::asynchronous_programming_in_rust::dbgprint;

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

impl TimerFuture {
    pub fn new(duration: Duration ) -> Self {
        dbgprint("TimerFuture::new", "started");
        let shared_state = Arc::new(Mutex::new(SharedState{
            completed: false,
            waker: None,
        }));

        let thread_shared_state = shared_state.clone();
        dbgprint("TimerFuture::new", "spawned a new thread");
        thread::spawn(move || {
            dbgprint("New Thread", "created");
            dbgprint("New Thread", "is going to sleep");
            thread::sleep(duration);
            dbgprint("New Thread", "get up!");
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                dbgprint("New Thread", "is going to call wake");
                waker.wake()
            }
            dbgprint("New Thread", "destoryed");
        });
        dbgprint("TimerFuture::new", "finished");
        TimerFuture { shared_state }
    }
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        dbgprint("TimerFuture::poll", "started");
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            dbgprint("TimerFuture::poll", "future ready");
            Poll::Ready(())
        } else {
            dbgprint("TimerFuture::poll", "future pending");
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}