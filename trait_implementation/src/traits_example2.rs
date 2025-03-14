use aggregator::default_impl::Summary;
use crate::estruct_data;
pub fn example2(){

    let array_data = ["Dev","Why is it hard","USA","The NPC's code used an older library!!"];
    let article  = estruct_data::make_article(array_data);
    //Uses default implementation
    println!("1 new Article:\n{}",article.summarize());

    let tweet = estruct_data::make_tweet("weather station","Cloudy");
    //Uses override implementation
    println!("1 new Tweet:\n{}",tweet.summarize());
    
    let blog = estruct_data::simple_blog();
    //Uses default implementation
    println!("1 new Blog post:\n{}",blog.summarize());
}