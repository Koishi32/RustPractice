use aggregator::default_impl_methods::Summary;
use crate::estruct_data;
pub fn example3(){

    println!("This trait calls a method");
    let tweet = estruct_data::make_tweet("weather station","Cloudy");
    println!("1 new tweet:\n{}",tweet.summarize());
    //The method summarize_author() was called by the default implementation 
    //of tweet
}