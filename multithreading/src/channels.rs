use std::sync::mpsc;
use std::thread;

pub fn channel_main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); // if receiver died then it will result in error
    });

    let received = rx.recv().unwrap();
    println!("got: {}", received);

}
pub fn mpsc_main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move ||{
            let vals = vec![String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread")];

            for val in vals {
                tx.send(val).unwrap();
            }
    });

    thread::spawn(move ||{
            let vals = vec![String::from("bye"),
            String::from("from-2"),
            String::from("the-2"),
            String::from("thread-2")];

            for val in vals {
                tx1.send(val).unwrap();
            }
    });


    for received in rx {
        println!("{}",received);
    }
}
