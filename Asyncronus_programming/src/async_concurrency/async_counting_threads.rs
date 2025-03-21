extern crate trpl; // required for mdbook test
use std::time::Duration;

pub fn new_counting_threads() {
    trpl::run(async {
        let handle = trpl::spawn_task(async {
            for i in 1..=10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(1000)).await;
        }
            handle.await.unwrap();
    });
}

pub fn one_join_two_futures_into_a_tupple(){
    trpl::run(
        async {
            let fut1 = async {
                for i in 1..10{
                    println!("hi number {i} from the first task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };
            let fut2 = async {
                for i in 1..5{
                    println!("hi number {i} from the second task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };
            trpl::join(fut1,fut2).await;
        });
}
// second task doesn't run unless we use await since futures futures do not execute until they are awaited
pub fn variation_without_async_block1(){
    trpl::run(
        async {
            let fut1 =  {
                for i in 1..10{
                    println!("hi number {i} from the first task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };
            let fut2 = async {
                for i in 1..5{
                    println!("hi number {i} from the second task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };
            fut2.await;
        });
}

// first task doesn't run unless we use await since futures futures do not execute until they are awaited
pub fn variation_without_async_block2(){
    trpl::run(
        async {
            let fut1 = async {
                for i in 1..10{
                    println!("hi number {i} from the first task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };
            
            let fut2 =  {
                for i in 1..5{
                    println!("hi number {i} from the second task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };
            fut1.await;
        });
}

//They are executed inmediatley one after another
pub fn variation_without_async_block3(){
    trpl::run(
        async {
            let fut1 =  {
                for i in 1..10{
                    println!("hi number {i} from the first task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };
            let fut2 =  {
                for i in 1..5{
                    println!("hi number {i} from the second task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };
        });
}

// the first task is completed then the second task is completed, they don't overlap
pub fn variation_await_each_async(){
    trpl::run(
        async {
            let fut1 = async {
                for i in 1..10{
                    println!("hi number {i} from the first task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };
            fut1.await;
            let fut2 = async {
                for i in 1..5{
                    println!("hi number {i} from the second task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };
            fut2.await;
        });
}
//Only fut1 runs since the fut2 isn't awaited
pub fn variation_first_loop(){
    trpl::run(
        async {
            let fut1 = async {
                for i in 1..10{
                    println!("hi number {i} from the first task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };
            
            let fut2 = async {
                for i in 1..5{
                    println!("hi number {i} from the second task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };
            fut1.await;
        });
}