use aggregator::trait_impl::{self,Summary};

pub fn example1(){
    let tweet = trait_impl::estruct_used::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    //println!("1 new tweet:\n{}", aggregator::lib_default_impl::Summary::summarize(&tweet));
    println!("1 new tweet:\n{}",tweet.summarize());

    let article  = trait_impl::estruct_used::NewsArticle{
        author :String::from("Game_Dev") ,
        headline :String::from("NPC Does nothing"),
        location : String::from("At home"),
        content : String::from("The Dev can't get his NPCs to do anything at the momment"),
    };
    //println!("1 new Article\n{}", aggregator::lib_trait_impl::Summary::summarize(&article));
    println!("1 new article:\n{}",article.summarize());

}