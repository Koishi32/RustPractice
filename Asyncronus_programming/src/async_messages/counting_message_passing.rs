extern crate trpl;
use std::time::Duration;

use tokio::sync::mpsc;
use tokio::runtime::Runtime;
use trpl::sleep;

pub fn series_of_messages_multiple_producer_channel(){
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();
        // a clone from tx is made to use multiple producer channels
        // and it's moved to the async block, once the block
        //finishes delivering messages the clone is dropped
        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        //org tx , it's moved into async block and it's dropped 
        //one the block finishies delivering messages
        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };
        // it takes messages from two tx and closes it's channel
        //once both producers are dropped
        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        trpl::join!(tx1_fut, tx_fut, rx_fut);
    });
}

pub fn series_of_sleeping_messages_await(){
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();
            //once the async bloke is finished, thanks to move,
            // tx will be dropped making the channel close
        let tx_fut = async move{
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };
        //now that thet futures are in their own ASYnc block
        //they can run on their own
        //tx sends messages every n milliseconds while rx is used
        //to print whatever messages come
        trpl::join(tx_fut, rx_fut).await;
    });
}


pub fn message_comparison_tokoi_thread_vs_future_await(){
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();
        println!("show me tokoi??");
        tokio_example().await;
        sleep(Duration::from_secs(10)).await;

        let val = String::from("hi");
        tx.send(val).unwrap();
        // Using a future to reviece a message without using a thread
        let received = rx.recv().await.unwrap();
        println!("Show async message");
        println!("Got: {received}");
        
    });
}
//JUst to test the crate using a spawner thread
pub async fn tokio_example(){
    let (tx, mut rx) = mpsc::channel(32);
    
    tokio::spawn(async move {
        sleep(Duration::from_secs(3)).await;
        tx.send("Message from tokio").await.unwrap();
    });

    let received = rx.recv().await.unwrap();
    println!("Received from tokio: {}", received);
}