
#[derive(Debug)]
enum UsState{
    Albama,
    Alaska,
    Arizona,
    California,
}

enum Coin{
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}



fn value_in_coin(coin:Coin) -> u8{
    match coin{
        Coin::Penny => {
            println!("heh, it's just a dime");
            5
        },
        Coin::Nickle => 10,
        Coin::Dime => 25,
        Coin::Quarter(a_state) => {
            println!("The state is {:?}", a_state);
            35
        }
    }
}
fn main() {
    let some_numb = Some(5);
    let some_string = Some(String::from("This is a string"));
    let x :i8 = 4;
    let y : Option<i8> = Some(19);

    // let sum = y.unwrap() + some_numb.unwrap() + x;
    let sum = y.unwrap_or(0) + some_numb.unwrap_or(0) + x;
    // println!("{}", some_numb);





    //-------- Match Expression -----------------//
    value_in_coin(Coin::Quarter(UsState::Arizona));

    let five = Some(5);
    let sex = plust_one(five);
    let seven = plust_one(None);


    let some_value = Some(3);
    match some_value{
        Some(5) => println!("Not, it's not 3"),
        Some(3) => println!("Yes, it is 3"),
        _ => (),
    }

    
}

fn plust_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i)=>{
            Some(i+1)
        },
    }
}