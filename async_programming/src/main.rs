mod game;

use futures::{
    future::{BoxFuture,FutureExt},
    task::{waker_ref, ArcWake}
};
use std::{
    future::Future,
    sync::mpsc::{sync_channel, Receiver, SyncSender},
    sync::{Arc, Mutex},
    task::Context,
    time::Duration,
};
use timer_future::TimerFuture;
/// Task executor that receives tasks off of a channel and runs them.
struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

/// `Spawner` spawns new futures onto the task channel.
#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

/// A future that can reschedule itself to be polled by an `Executor`.
struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    /// Handle to place the task itself back onto the task queue.
    task_sender: SyncSender<Arc<Task>>,
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor{ready_queue}, Spawner{task_sender})
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender:self.task_sender.clone() 
        });
        self.task_sender.send(task).expect("too many task queued");
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self
        .task_sender
        .send(cloned)
        .expect("Too many task queued");
    }
}

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&waker);
                if future.as_mut().poll(context).is_pending() {
                    *future_slot = Some(future);
                }

            }

        }
    }
}

fn main() {
 let (executor, spawner) = new_executor_and_spawner();


 spawner.spawn(async {
    println!("la");
    TimerFuture::new(Duration::new(2,0)).await;
    println!("la");
 });

 spawner.spawn(async {
    println!("this is test");
    TimerFuture::new(Duration::new(4,0)).await;
    println!("done!");
 });

 drop(spawner);
 for i in 1..100 {
    println!("{}",i);
 }
 executor.run();
 println!("this is stet");
}
