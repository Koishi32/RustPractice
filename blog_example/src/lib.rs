use core::str;
pub mod enum_lib;
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {

    pub fn approve(&mut self){
        if let Some(s) = self.state.take(){
            self.state = Some(s.approve())
        }
    }
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn request_review(&mut self){
        if let Some(s) = self.state.take(){
            self.state=Some(s.request_review())
        }
    }
    pub fn add_text(&mut self, text:&str){
        let res = self.state.as_ref().unwrap().add_text(text);
        self.content.push_str(&res);
    }
    pub fn content(&self) -> &str{
        self.state.as_ref().unwrap().content(self)
    }
    pub fn reject(&mut self){
        if let Some(s) = self.state.take(){
            self.state=Some(s.reject())
        }
    }

}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn add_text<'a>(&self,text:&'a str) -> &'a str;
    fn approve(self:Box<Self>)-> Box<dyn State>;
    fn reject (self:Box<Self>)-> Box<dyn State>;
    fn content<'a>(&self,post:&'a Post) -> &'a str{
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn approve(self:Box<Self>)-> Box<dyn State> {
        self
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview{})
    }
    fn reject (self:Box<Self>)-> Box<dyn State> {
        self
    }
    fn add_text<'a>(&self,text:&'a str)-> &'a str{
        text
    }

}

struct PendingReview{}

impl State for PendingReview{
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self:Box<Self>)-> Box<dyn State> {
        Box::new(PreAproved{})
    }
    fn reject (self:Box<Self>)-> Box<dyn State> {
        Box::new(Draft{})
    }
    fn add_text<'a>(&self,text:&'a str) -> &'a str{
        ""
    }
}

struct Published{}

impl State for Published{
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self:Box<Self>)-> Box<dyn State> {
        self
    }
    fn reject (self:Box<Self>)-> Box<dyn State> {
        self
    }
    fn add_text<'a>(&self,text:&'a str) -> &'a str{
        ""
    }
    fn content<'a>(&self,post:&'a Post) -> &'a str {
        &post.content
    }
}

struct PreAproved{}

impl State for PreAproved {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self:Box<Self>)-> Box<dyn State> {
        Box::new(Published{})
    }
    fn reject (self:Box<Self>)-> Box<dyn State> {
        self
    }
    fn add_text<'a>(&self,text:&'a str) -> &'a str{
        ""
    }
}