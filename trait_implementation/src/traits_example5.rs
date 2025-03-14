use aggregator::traits_as_parameters::notify;
use aggregator::trait_as_return_type;
pub fn example5(){
 let new_tweet = trait_as_return_type::returns_summarizable();
 //println!("Summary: {}",new_tweet.summarize());
 notify(&new_tweet);
}