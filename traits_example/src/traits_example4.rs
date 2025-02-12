
use aggregator::traits_as_parameters::notify;

use crate::estruct_data;

use aggregator::trait_bound::notify_generic;
use aggregator::trait_bound::notify_any_summary_type;
use aggregator::trait_bound::notify_specific_summary_type;
use aggregator::trait_bound::notify_with_multiple_traits;

pub fn example4(){
    println!("The next items will be have their trait executed by another function");
    
    let tweet = estruct_data::make_tweet("Trash Collection","Trucks are unavaliable");
    
    let blog_tupple_data = ("Hanging on","Job Hunting","Still looking for a job, it's hard tbh");
    let new_blog = estruct_data::make_blog(blog_tupple_data);
    
    let array_info = ["Office of goverment","Crisis at the capital","Capital","The economic crisis causes lots of lay offs"];
    let article = estruct_data::make_article(array_info);

    notify(&tweet);
    notify(&new_blog);
    notify(&article);

    println!("\nTrait_bound_syntaxt");
    notify_generic(&article);
    notify_any_summary_type(&tweet,&article);

    let info_tupple = ("Daily Life Update","Job Found","Finally found something to work on");
    let another_blog = estruct_data::make_blog(info_tupple);
    //The next function shouldn't run since the syntaxt checks
    //if they are the same data type
    //notify_specific_summary_type(&another_blog,&tweet);
    //The next one compiles since they are the same Data type
    notify_specific_summary_type(&new_blog,&another_blog);

    println!("\nMultiple trait bounds with Display and summary");

    let tweet = estruct_data::simple_tweet();
    notify_with_multiple_traits(&tweet);

}