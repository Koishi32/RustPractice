use aggregator::lib_default_impl::{self,Summary};

pub fn example2(){

    let article  = lib_default_impl::estruct_used::NewsArticle{
        author :String::from("Dev") ,
        headline :String::from("Does nothing"),
        location : String::from("At Work"),
        content : String::from("The Dev won't work"),
    };
    //println!("1 new Article:\n{}",aggregator::lib_default_impl::Summary::summarize(&article));
    println!("1 new Article:\n{}",article.summarize());
    
    let tweet = lib_default_impl::estruct_used::Tweet {
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