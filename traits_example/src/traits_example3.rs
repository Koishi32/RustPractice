use aggregator::default_impl_methods::{self,Summary};

pub fn example3(){
    println!("This trait calls a method");
    let tweet = default_impl_methods::estruct_used::Tweet {
        username: String::from("weather station"),
        content: String::from(
            "Cloudy",
        ),
        reply: false,
        retweet: false,
    };
    //println!("1 new tweet:\n{}",aggregator::lib_default_impl::Summary::summarize(&tweet));
    println!("1 new tweet:\n{}",tweet.summarize());
    
    

}