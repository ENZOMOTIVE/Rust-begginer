// To check if a number is even or not

fn main() {
    let a = 10;
    println!("{}",is_even(a));
    
}

fn is_even(num: i32) -> bool {
    
    if num%2 == 0 {
        return true;
    }
    else {
        return false;
    }
}
