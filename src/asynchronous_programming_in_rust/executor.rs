use futures::{
    future::{BoxFuture, FutureExt},
    task::{waker_ref, ArcWake},
};

use std::{
    future::Future,
    sync::mpsc::{sync_channel, Receiver, SyncSender},
    sync::{Arc, Mutex},
    task::Context,
};

use super::dbgprint;

pub struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

#[derive(Clone)]
pub struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
}

pub fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAXED_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAXED_QUEUED_TASKS);
    return (Executor { ready_queue }, Spawner { task_sender });
}

impl Spawner {
    pub fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many tasks queued");
    }
}
impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued");
    }
}

impl Executor {
    pub fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {

            dbgprint("Executor::run", "received a task");

            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {

                dbgprint("Executor::run", "task has a future");

                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);
                let future_temp = future.as_mut();

                dbgprint("Executor::run", "started to poll a future");

                let output = future_temp.poll(context);

                dbgprint("Executor::run", "ended in polling a future");

                if output.is_pending() {
                    dbgprint("Executor::run", "future is pending");
                    *future_slot = Some(future);
                } else {
                    dbgprint("Executor::run", "future is finished");
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::asynchronous_programming_in_rust::dbgprint;

    use super::super::timer_future::TimerFuture;
    use super::new_executor_and_spawner;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn executor_test() {
        let (executor, spawner) = new_executor_and_spawner();

        spawner.spawn(async {
            dbgprint("Anonymous async block","started");
            println!("howdy!");
            TimerFuture::new(Duration::new(2, 0)).await;
            dbgprint("Anonymous async block","going to sleep");
            thread::sleep(Duration::new(2,0));
            dbgprint("Anonymous async block","get up!");
            println!("down!");
            dbgprint("Anonymous async block","finished");
        });

        drop(spawner); // how does executor knows it is finished?

        executor.run();
    }

}
