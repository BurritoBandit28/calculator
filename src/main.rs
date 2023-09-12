use std::io;

fn main() {
    println!("whats poppin biatch");
    println!("okay broskie its time to do some MATH (wow cool I love that)\n");
    calcYouLaterLol();
}

//TODO - take this more seriously
fn calcYouLaterLol(){
    println!("\nOkay so like, do you want to add (+), subtract (-), multiply (*) or divide (/)? type the symbol pls \n + - * /");

    let mut op : String = String::new();
    let mut n1 : String = String::new();
    let mut n2 : String = String::new();

    io::stdin()
        .read_line(&mut op)
        .expect("m8 you messed up somehow");

    println!("alright buddy, time to choose ur numbers, enter the first and then the second");

    io::stdin()
        .read_line(&mut n1)
        .expect("imbicile");

    io::stdin()
        .read_line(&mut n2)
        .expect("imbicile");

    let n1 : u32 = n1.trim().parse::<u32>().unwrap();
    let n2 : u32 = n2.trim().parse::<u32>().unwrap();

    if (n1 == 0u32 || n2 == 0u32) && op.chars().next().unwrap() == '/' {
        println!("you messed up dude, no dividing with 0 you scallywag");
        calcYouLaterLol();
        return;
    }

    match op.chars().next().unwrap() {
        '+' => println!("gotcha {} + {} = {}", n1, n2, n1 + n2),
        '-' => println!("you da man, man {} - {} = {}", n1, n2, n1 - n2),
        '*' => println!("woah thats MAD {} * {} = {}", n1, n2, n1 * n2),

        //btw I just learnt rust has no try-catch trying some divide by zero prevention
        '/' => println!("what the actual fuck jesus christ {} / {} = {}", n1, n2, n1 / n2),
        _ => println!("dude you broke it (use one of the four symbols only)")
    }
}
