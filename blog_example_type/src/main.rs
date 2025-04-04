use blog_example_type::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    let approved_post= post.post();
    assert_eq!("I ate a salad for lunch today", approved_post.content());

    println!("Sucesess content is:\n{}",approved_post.content());
}