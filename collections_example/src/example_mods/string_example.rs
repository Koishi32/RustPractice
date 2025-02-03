pub fn string_examples(){
        println!("STRINGS EXAMPLE\n");
        assing_string();
        handle_unico_for_strings();
        push_in_strings();
        concatenation_with_operator();
        print_string_format();
        rust_string_handling();
        proper_iteration();
    }

fn assing_string(){
        let mut s = String::new();
        let data = "initial contents";
        let s = data.to_string();
        println!("{}",s);
        // the method also works on a literal directly:
        let s = "initial contents".to_string();
        let s = String::from("initial contents");
        println!("{}",s);
    }

fn handle_unico_for_strings(){
        let mut hello = String::from("السلام عليكم");
        let hello = String::from("Dobrý den");
        let hello = String::from("Hello");
        let hello = String::from("שלום");
        let hello = String::from("नमस्ते");
        let hello = String::from("こんにちは");
        let hello = String::from("안녕하세요");
        let hello = String::from("你好");
        let hello = String::from("Olá");
        let hello = String::from("Здравствуйте");
        let hello = String::from("Hola");
        println!("{}",hello);
        
    }
fn push_in_strings(){
        let mut s = String::from("foo");
        s.push_str("bar");
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2 is {s2}");
        let mut s = String::from("lo");
        s.push('l');
    }
fn concatenation_with_operator(){
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        // The add func used by the + operator has taken ownership of the s1 car
        // it will be moved to s3 and s1 can't be used after
        //since the + operator only needed a reference of the second string (s2)
        //s2 an still be used
        let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
        println!("{}",s3);
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = s1 + "-" + &s2 + "-" + &s3;
        println!("with operator: {}",s);
    }

fn print_string_format(){
         // Better way to print multiple strings without taking ownership of any of them
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{s1}-{s2}-{s3}");
        println!("with format: {}",s);
    }

fn rust_string_handling(){
        let long_mesg = String::from("Remainder that rust doesn't implement string indexation since it handles strings diferently in memory
                let s1 = String::from(hello);
                let h = s1[0];
                the previous line makes Rust panic");
        println!("{}",long_mesg);
let long_mesg2 = String::from("
handling this string with string slice ranges 
since each one of these characters is two bytes
s will be 3д
a slice like [0..1] will cause Rust to panic
since we are not respecting char boundaries");
println!("{}",long_mesg2);
        
        let hello = "Здравствуйте";
        let s = &hello[0..4];
        println!("By slice (0..4): {}", s);
    }

fn proper_iteration(){
    let mesg = String::from("Proper way of iterating over strings: ");
    println!("{}",mesg);
    let hello = "Здравствуйте";
        println!("By character");
        // by character
        for c in hello.chars() {
        println!("{c}");
        }
        println!("By their byte UTF-8 representation: ");
        // by raw byte
        for b in hello.bytes() {
        println!("{b}");
        }    
}