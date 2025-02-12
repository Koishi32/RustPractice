use aggregator::default_impl::{self,Summary};

pub fn example2(){

    let article  = default_impl::estruct_used::NewsArticle{
        author :String::from("Dev") ,
        headline :String::from("Does nothing"),
        location : String::from("At Work"),
        content : String::from("The Dev won't work"),
    };
    //Uses default implementation
    println!("1 new Article:\n{}",article.summarize());
    
    let tweet = default_impl::estruct_used::Tweet {
        username: String::from("weather station"),
        content: String::from(
            "Cloudy",
        ),
        reply: false,
        retweet: false,
    };
    //Uses override implementation
    println!("1 new tweet:\n{}",tweet.summarize());
    
    let blog_post = default_impl::estruct_used::Blog{
        tittle : String::from("The worst day of my life"),
        sub_tittle : String::from("Update"),
        content : String::from("So i got fired today"), 
    };
    //Uses default implementation
    println!("1 new blog post:\n{}",blog_post.summarize());
}