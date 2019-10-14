/// Accept twointeger values form the user
///
/// main.rs
/// 
/// # Examples
///
/// ```
/// use std::io;
/// extern crate check;
/// use check::check as chk;
/// fn main() {
///     println!("Please Enter Numerator");
///     let mut Numerator = String::new();
///     io::stdin().read_line(&mut Numerator);
///     let Numerator:i32 = Numerator.trim().parse().unwrap();
///
///     println!("Please Enter Denominator");
///     let mut Denominator = String::new();
///     io::stdin().read_line(&mut Denominator);
///     let Denominator:i32 = Denominator.trim().parse().unwrap();
///    
///     chk::check(Numerator,Denominator);
///     
/// }
/// ```
use std::io;
extern crate check;
use check::check as chk;
fn main() {
    println!("Please Enter Numerator");
    let mut Numerator = String::new();
    io::stdin().read_line(&mut Numerator);
    let Numerator:i32 = Numerator.trim().parse().unwrap();

    println!("Please Enter Denominator");
    let mut Denominator = String::new();
    io::stdin().read_line(&mut Denominator);
    let Denominator:i32 = Denominator.trim().parse().unwrap();

    chk::check(Numerator,Denominator);
}