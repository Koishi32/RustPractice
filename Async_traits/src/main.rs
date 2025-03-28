#![allow(unused)]
fn main() {
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
}
use std::pin::Pin;
use std::task::{Context, Poll};

trait Stream {
    type Item;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Option<Self::Item>>;
}

trait StreamExt: Stream {
    async fn next(&mut self) -> Option<Self::Item>
    where
        Self: Unpin;

    // other methods...
}

/*

// Safe to move (Unpin)
struct SafeType {
    data: i32,
}

// Must stay pinned (!Unpin)
struct UnsafeType {
    ptr: *const i32,
}

impl !Unpin for UnsafeType {} // Manual negation


pub fn use_pin(){
// Usage with Pin
    let pinned = Box::pin(UnsafeType { ptr: &42 }); // Must stay pinned
    let safe = Box::pin(SafeType { data: 42 }); // Can be moved via Pin::into_inner()
}
*/
