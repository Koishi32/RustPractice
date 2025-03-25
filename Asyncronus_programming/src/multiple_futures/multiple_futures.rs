extern crate trpl;
use std::{
    process::Output,
    time::Duration,
    future::Future,
    pin::{pin,Pin}
};

pub fn series_of_messages_all_futures(){
    trpl::run(async {
        //using pin to wrap around our futures and give them the type
        //pin and trait unpin
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();

        let tx1_fut = pin!(async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_secs(1)).await;
            }
        });

        let rx_fut = pin!(
            async {
                while let Some(value) = rx.recv().await {
                    println!("received '{value}'");
                }
            }
        );

        let tx_fut =  pin!(
            async move {
                let vals = vec![
                    String::from("more"),
                    String::from("messages"),
                    String::from("for"),
                    String::from("you"),
                ];
    
                for val in vals {
                    tx.send(val).unwrap();
                    trpl::sleep(Duration::from_secs(1)).await;
                }
            }
        );
        //We can use box<> and pin each one, 
        //but we don't really need to 
        //allocate on the heap
        /* 
        let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
            vec![Box::pin(tx1_fut), Box::pin(rx_fut), Box::pin(tx_fut)];
        */
        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> =
            vec![tx1_fut, rx_fut, tx_fut];
        trpl::join_all(futures).await;
    });
}

pub fn example_futures_diferent_type(){
    trpl::run(async {
        let a = async { 1u32 };
        let b = async { "Hello!" };
        let c = async { true };

        let (a_result, b_result, c_result) = trpl::join!(a, b, c);
        println!("{a_result}, {b_result}, {c_result}");
    });
}