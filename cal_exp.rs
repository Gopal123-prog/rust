fn main(){
    let a = 10;
    let b = 20;
    let op = "+";
    match op{
        "*" => println!("{} * {} = {}",a,b,a*b),
        "+" => println!("{} + {} = {}",a,b,a+b),
        "-" => println!("{} - {} = {}",a,b,a-b),
        "/" => println!("{} / {} = {}",a,b,a/b),
        _ => println!("Invalid operator"), 
    }
}