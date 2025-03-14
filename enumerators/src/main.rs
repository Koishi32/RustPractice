enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
    FakeCoin,
    ToyCoin,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
	   Coin::ToyCoin =>{
		println!("A toy coin");
		1
	}
	_=>{
		println!("Non valid coin");
		0
	}
    }
}

fn main (){
    let my_coin = Coin::Dime;
    println!("{}",value_in_cents(my_coin));
    let my_coin = Coin::Nickel;
    println!("{}",value_in_cents(my_coin));
    let my_coin = Coin::Quarter;
    println!("{}",value_in_cents(my_coin));
    let my_coin = Coin::Penny;
    println!("{}",value_in_cents(my_coin));
    let my_coin = Coin::ToyCoin;
    println!("{}",value_in_cents(my_coin));
    let my_coin = Coin::FakeCoin;
    println!("{}",value_in_cents(my_coin));
    
}
