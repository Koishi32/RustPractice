#![allow(unused)]
mod async_syntax;
mod async_concurrency;
mod async_messages;
mod multiple_futures;
mod custom_async_abstractions;
mod futures_sequence;
fn main() {
   //async_syntax::basic_web_scrapping::basic_web_scrapping();
   //async_concurrency::async_counting_threads::new_counting_threads();
   //async_concurrency::async_counting_threads::new_counting_threads();
   //async_concurrency::async_counting_threads::one_join_two_futures_into_a_tupple();
  /* 
  println!("\nVariation 1");
   async_concurrency::async_counting_threads::variation_without_async_block1();
   println!("\nVariation 2");
   async_concurrency::async_counting_threads::variation_without_async_block2();
   println!("\nVariation 3");
   async_concurrency::async_counting_threads::variation_without_async_block3();
   println!("\nVariation 4");
   async_concurrency::async_counting_threads::variation_await_each_async();
   println!("\nVariation 5");
   async_concurrency::async_counting_threads::variation_first_loop();
  */
   //async_messages::counting_message_passing::message_comparison_tokoi_thread_vs_future_await();
   //async_messages::counting_message_passing::series_of_sleeping_messages_await();
   //async_messages::counting_message_passing::series_of_messages_multiple_producer_channel();
   //async_messages::multiple_futures::example_futures_diferent_type();
   //async_messages::multiple_futures::series_of_messages_all_futures();
   //async_messages::counting_message_passing::series_of_messages_multiple_producer_channel();
   //multiple_futures::multiple_futures::series_of_messages_all_futures();
   //multiple_futures::multiple_futures::example_futures_diferent_type();
   //multiple_futures::racing_futures::racing_futures();
   //multiple_futures::control_runtime::call_slow_to_return_control_to_runtime();
   //multiple_futures::control_runtime::bench_mark_test(); /// Spoiler yield wins!!!
   //custom_async_abstractions::custom_timeout::my_timeout();
   //futures_sequence::stream_creation::creating_stream();
   futures_sequence::web_socket_protocol::web_socket();
}