enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
struct Point {
    w: i32,
    q: i32,
    x: i32,
    y: i32,
    z: i32,
    asd:i32,
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

pub fn multiple_patterns(){
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

}

pub fn range_patterns(){
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

}

pub fn range_char_pattern(){
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

}

pub fn destruct_pattern() {
    let p = crate::Point { x: 0, y: 7 };

    let crate::Point { x:a, y: b } = p;
    let new_var=a+2;
    assert_eq!(0, a);
    assert_eq!(7, b);
    println!("{new_var}");
}

pub fn destruct_simple_pattern(){
    let p = crate::Point { x: 0, y: 7 };

    let crate::Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}

pub fn destruct_especific_pattern(){
    let p = crate::Point { x: 3, y: 7 };

    match p {
        crate::Point { x, y: 0 } => println!("On the x axis at {x}"),
        crate::Point { x: 0, y } => println!("On the y axis at {y}"),
        crate::Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}

pub fn enum_pattern() {
    let msg = Message::Move{x:160, y:255};
    let msg2 = Message::Write("WordsTest".to_string());
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(r, g, b)) => {
            println!("Change the color to red {r}, green {g}, and blue {b}");
        }
    }
    match msg2 {
        Message::Write(texto) => {
            println!("Text detected {texto}");
        }
        _ => {
            println!("Nothing");
        }
    }
}

pub fn nested_enum_pattern(){
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        }
        _ => (),
    }
}

pub fn ignore_nested_enum_part_pattern() {
    let mut setting_value = None;
    let new_setting_value = Some(2);

    println!("setting was {setting_value:?}");

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {setting_value:?}");
    println!("new setting is {new_setting_value:?}");

}

pub fn ignore_part_tupple_pattern(){
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }
}

pub fn skipping_pattern(){
    let origin = Point { w: 1, q:43, x: 2, y: 3, z: 4, asd:123 };

    match origin {
        Point { w,x,asd, .. } => println!("w is {w} x is {x} asd is {asd}"),
    }
}

pub fn skipping_tupple_pattern(){
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (a, .., z) => {
            println!("Some numbers: {a}, {z}");
        }
    }
}

pub fn match_guard_pattern(){
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    let num = Some(3);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }
}

pub fn match_guard_pattern_shadow_fix(){
    let x = Some(5);
    let y = 5;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");
}

pub fn match_guard_or_operator(){
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
    let x = 4;
    let y = true;

    match x {
        4 | 5 | 6 if y => println!("yes"), // the condition if applies to all matches
        _ => println!("no"),
    }
}

pub fn bindings(){
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg { //@ Test the value and makes it avaiable for use
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {id}"),
    }
}