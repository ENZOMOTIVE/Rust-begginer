// fibonacci of a number it takes as input

fn main(){
    println!("{}", fib(10));
}

fn fib(num: i32) -> i32{
    let mut first = 0;
    let mut second = 1;
    if num == 0 {
        return first;
    }
    if num == 1 {
        return second;
    }
    
    for _ in 0..(num - 1) {
        let temp = second;
        second = second + first;
        first = temp;
    }
    return second;
}




// * Note: Any variable we define in rust by default is a constant
// mut means the thing which is not constant
