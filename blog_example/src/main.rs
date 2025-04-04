use blog_example::Post;
use blog_example::enum_lib::Post as Post_enum;
fn main() {
    trait_object_implementation();
    enum_implementation();
    new_implementation();
}

fn new_implementation(){
    let mut post = Post::new();

    post.add_text("Trying update");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.request_review();

    post.approve();
    assert_eq!("", post.content());

    post.approve();

    post.add_text("The string shouldn't change");
    assert_eq!("Trying update", post.content());

    println!("SUCESS WITH TRAIT OBJECT, REJECT, APPROVE AND DRAFT TEST:\n{}",post.content());
}

fn trait_object_implementation(){
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    println!("SUCESS WITH TRAIT OBJECT\n{}",post.content());
    
}

fn enum_implementation(){
    let mut post = Post_enum::new();

    post.add_text("I ate a Eggs for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a Eggs for lunch today", post.content());

    println!("SUCESS WITH ENUM\n{}",post.content());
}