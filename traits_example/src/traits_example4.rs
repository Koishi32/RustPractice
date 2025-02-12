use aggregator::traits_as_parameters::notify;
use aggregator::traits_as_parameters::trait_impl;

use aggregator::trait_bound::notify_generic;
use aggregator::trait_bound::notify_any_summary_type;
use aggregator::trait_bound::notify_specific_summary_type;

pub fn example4(){
    println!("The next items will be have their trait executed by another function");
    let tweet = trait_impl::estruct_used::Tweet {
        username: String::from("Trash Collection"),
        content: String::from(
            "Trucks are unavaliable",
        ),
        reply: false,
        retweet: false,
    };
    let new_blog = trait_impl::estruct_used::Blog{
        tittle: String::from("Hanging on"),
        sub_tittle: String::from("Job Hunting"),
        content: String::from("Not a lot of good options for now tbh"),
    };
    let article = trait_impl::estruct_used::NewsArticle{
        author: String::from("Office of goverment"),
        headline : String::from("Crisis at the capital"),
        location:String::from("Capital"),
        content : String::from("The economic crisis causes lots of lay offs"),
    };
    notify(&tweet);
    notify(&new_blog);
    notify(&article);

    println!("\nTrait_bound_syntaxt");
    notify_generic(&article);
    notify_any_summary_type(&tweet,&article);


    let another_blog = trait_impl::estruct_used::Blog{
        tittle: String::from("Daily Life Update"),
        sub_tittle: String::from("Job Found"),
        content: String::from("Finally found something to work on"),
    };
    //The next function shouldn't run since the syntaxt checks
    //if they are the same data type
    //notify_specific_summary_type(&another_blog,&tweet);
    //The next one compiles since they are the same Data type
    notify_specific_summary_type(&new_blog,&another_blog);

}