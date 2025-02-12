use aggregator::trait_impl::Summary;
use crate::estruct_data;
pub fn example1(){
    let tweet = estruct_data::simple_tweet();
    println!("Tweet:\n{}",tweet.summarize());

    let article = estruct_data::simple_article();
    println!("Article:\n{}",article.summarize());

    let blog = estruct_data::simple_blog();
    println!("Blog:\n{}",blog.summarize());
}