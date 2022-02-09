#![allow(dead_code)]

use std::future::Future;

fn main() {
    println!("Hello, world");
    
    let x = foo1();
    async {
            println!("{}",x.await);
            ()
       }
}

async fn foo1() -> usize {
    println!("hello");

    0
}

fn foo2() -> impl Future<Output = usize> {
    async { 
        0
    }
}





