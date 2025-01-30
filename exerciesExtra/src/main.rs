use std::io;
fn actual_count_fibonachi(num : i32){
    let mut p1 = 0;
    let mut p2 = 1;
    let mut res;
    for _a in 2..num{
        res = p1 + p2;
        println!("{res}");
        p1 = p2;
        p2 = res;
    }
}
fn main() {
    println!("Insert fibonachi sequence n: ");
    let mut user_n = String::new();
    io::stdin()
    .read_line(&mut user_n)
    .expect("Failure To read line");
    let user_n=user_n.trim();
    println!("Getting secuence: {user_n}");
    let n:i32 = user_n.parse().expect("Couldn't parse to int");
    if n==0{
        println!("0");
    }else if n==1{
        println!("0");
        println!("1");
    }else{
        println!("0");
        println!("1");
        actual_count_fibonachi(n);
    }
}