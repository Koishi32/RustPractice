// This enum purposely neither implements nor derives PartialEq.
// That is why comparing Foo::Bar == a fails below.
#[derive(Debug)]
enum Foo {Bar}

fn main() {
    simple_if_let();
    if_let_use();
    if_let_estructure();
    increments();
    while_let_simpler();
}

fn simple_if_let(){
    let a = Foo::Bar;

    // Variable a matches Foo::Bar
    if let Foo::Bar = a {
    // ^-- this causes a compile-time error. Use `if let` instead.
        println!("a is foobar {:?}",a);
    }
}

fn if_let_use() {
    // All have type `Option<i32>`
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // The `if let` construct reads: "if `let` destructures `number` into
    // `Some(i)`, evaluate the block (`{}`).
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // If you need to specify a failure, use an else:
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // Destructure failed. Change to the failure case.
        println!("Didn't match a number. Let's go with a letter!");
    }

    // Provide an altered failing condition.
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    // Destructure failed. Evaluate an `else if` condition to see if the
    // alternate failure branch should be taken:
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // The condition evaluated false. This branch is the default:
        println!("I don't like letters. Let's go with an emoticon :)!");
    }
}

// Our example enum
enum Foo2 {
    Bar,
    Baz,
    Qux(u32)
}

fn if_let_estructure() {
    // Create example variables
    let a = Foo2::Bar;
    let b = Foo2::Baz;
    let c = Foo2::Qux(100);
    
    // Variable a matches Foo::Bar
    if let Foo2::Bar = a {
        println!("a is foobar");
    }
    
    // Variable b does not match Foo::Bar
    // So this will print nothing
    if let Foo2::Bar = b {
        println!("b is foobar");
    }
    
    // Variable c matches Foo::Qux which has a value
    // Similar to Some() in the previous example
    if let Foo2::Qux(value) = c {
        println!("c is {}", value);
    }

    // Binding also works with `if let`
    if let Foo2::Qux(value @ 100) = c {
        println!("c is one hundred");
    }
}

fn increments(){
    let mut optional = Some(0);

// Repeatedly try this test.
loop {
    match optional {
        // If `optional` destructures, evaluate the block.
        Some(i) => {
            if i > 9 {
                println!("Greater than 9, quit!");
                optional = None;
            } else {
                println!("`i` is `{:?}`. Try again.", i);
                optional = Some(i + 1);
            }
            // ^ Requires 3 indentations!
        },
        // Quit the loop when the destructure fails:
        _ => { break; }
        // ^ Why should this be required? There must be a better way!
    }
}
}
// Make `optional` of type `Option<i32>`


// More compact than previous example
fn while_let_simpler() {
    // Make `optional` of type `Option<i32>`
    let mut optional = Some(0);
    
    // This reads: "while `let` destructures `optional` into
    // `Some(i)`, evaluate the block (`{}`). Else `break`.
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ Less rightward drift and doesn't require
        // explicitly handling the failing case.
    }
    // ^ `if let` had additional optional `else`/`else if`
    // clauses. `while let` does not have these.
}