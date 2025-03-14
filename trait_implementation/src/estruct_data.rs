use aggregator::estruct_used;

pub fn simple_tweet() -> estruct_used::Tweet{
    let tweet = estruct_used::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    tweet
 }
 pub fn simple_article() -> estruct_used::NewsArticle{
    let article  = estruct_used::NewsArticle{
        author :String::from("Game_Dev") ,
        headline :String::from("NPC Does nothing"),
        location : String::from("At home"),
        content : String::from("The Dev can't get his NPCs to do anything at the momment"),
    };
    article
 }
 pub fn simple_blog() ->estruct_used::Blog{
    let blog_post = estruct_used::Blog{
        tittle : String::from("The worst day of my life"),
        sub_tittle : String::from("Update"),
        content : String::from("So i got fired today"), 
    };
    blog_post
 }

 pub fn make_tweet(user:&str,content:&str) -> estruct_used::Tweet{
    let tweet = estruct_used::Tweet{
        username:user.to_string(),
        content:content.to_string(),
        reply: false,
        retweet: false,
    };
    tweet
 }

 pub fn make_article(array_info:[&str;4])-> estruct_used::NewsArticle{
    let article  = estruct_used::NewsArticle{
        author :array_info[0].to_string(),
        headline :array_info[1].to_string(),
        location :array_info[2].to_string(),
        content :array_info[3].to_string(),
    };
    article
 }

 pub fn make_blog(info_tupple:(&str,&str,&str)) ->estruct_used::Blog{
    let tittle = info_tupple.0.to_string();
    let sub_tittle = info_tupple.1.to_string();
    let content = info_tupple.2.to_string();
    let blog_post = estruct_used::Blog{
        tittle : tittle,
        sub_tittle :sub_tittle,
        content : content, 
    };
    blog_post
 }