use std::{
    task::{Context, Poll, Waker},
    time::{Duration, Instant},
    sync::{Arc, Mutex},
    pin::Pin,
    future::Future,
    thread,
};
use futures::{task::{self, ArcWake}};
use crossbeam::channel;

pub struct Delay {
    pub when: Instant,
    waker: Option<Arc<Mutex<Waker>>>,
}

impl Future for Delay {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<()>
    {
        if let Some(waker) = &self.waker {
            let mut waker = waker.lock().unwrap();
            if !waker.will_wake(cx.waker()) {
                *waker = cx.waker().clone();
            }
        } else {
            let when = self.when;
            let waker = Arc::new(Mutex::new(cx.waker().clone()));
            self.waker = Some(waker.clone());

            thread::spawn(move || {
                let now = Instant::now();

                if now < when {
                    thread::sleep(when-now);
                }

                let waker = waker.lock().unwrap();
                waker.wake_by_ref();
            });
        }

        if Instant::now() >= self.when {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

pub enum MainFuture {
    State0,
    State1(Delay),
    Terminated,
}

impl Future for MainFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<()>
    {
        use MainFuture::*;
        loop {
            match *self {
                State0 => {
                    let when = Instant::now() + Duration::from_millis(10);
                    let waker = Some(Arc::new(Mutex::new(cx.waker().clone())));
                    let future = Delay { when, waker };
                    *self = State1(future);
                }
                State1(ref mut my_future) => {
                    match Pin::new(my_future).poll(cx) {
                        Poll::Ready(out) => {
                            assert_eq!(out, ());
                            *self = Terminated;
                            return Poll::Ready(());
                        }
                        Poll::Pending => {
                            return Poll::Pending;
                        }
                    }
                }
                Terminated => {
                    panic!("future polled after completion")
                }
            }
        }
    }

}

pub struct MiniTokio {
    scheduled: channel::Receiver<Arc<Task>>,
    sender: channel::Sender<Arc<Task>>,
}

impl MiniTokio {
    pub fn new() -> MiniTokio {
        let (sender, scheduled) = channel::unbounded();
        MiniTokio { scheduled, sender}
    }

    pub fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output=()> + Send + 'static,
    {
        Task::spawn(future, &self.sender);
    }

    pub fn run(&mut self) {
        while let Ok(task) = self.scheduled.recv() {
            task.poll();
        }
    }
}

pub struct Task {
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    executor: channel::Sender<Arc<Task>>,
}

impl Task {
    pub fn schedule(self: &Arc<Self>) {
        self.executor.send(self.clone()).ok();
    }

    pub fn poll(self: Arc<Self>) {
        let waker = task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);

        let mut future = self.future.try_lock().unwrap();

        let _ = future.as_mut().poll(&mut cx);
    }

    pub fn spawn<F>(future: F, sender: &channel::Sender<Arc<Task>>) 
    where
        F: Future<Output=()> + Send + 'static,
    {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
            executor: sender.clone(),
        });

        let _ = sender.send(task);
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.schedule();
    }
}