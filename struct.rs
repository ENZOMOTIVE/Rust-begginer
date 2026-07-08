struct User {
    first_name: String,
    roll_number: i32,
    batch: i32,
    branch: String,
}

fn main() {
    let user = User {
        first_name: String::from("Aayushman"),
         roll_number: 114,
         batch: 2021,
         branch: String::from("CSE"),
    };
    println!("{}", user.first_name);
     println!("{}", user.roll_number);
      println!("{}", user.batch);
         println!("{}", user.branch);
   }
