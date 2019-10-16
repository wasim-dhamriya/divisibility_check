pub mod check
{
   pub fn check(x:i32,y:i32)
    {
        if x % y == 0
    {
        println!("Number {} is Completely divisible by {}",x,y);
    }
    else
    {
        println!("Number {} is not Completely divisible by {}",x,y);
    }
    }
}
