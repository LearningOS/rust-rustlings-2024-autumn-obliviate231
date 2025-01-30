// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.

struct Counter{
    count: u64,
}

impl Counter{
    fn new(value: u64) -> Counter{
        Counter{count: value}
    }
}
impl Iterator for Counter{
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item>{
        if self.count > 1 {self.count -= 1; Some(self.count)}
        else {None}
    }
}


pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    if num == 0 {return 1;}
    Counter::new(num).fold(num, |acc, x| acc * x)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        println!("{}", factorial(2));
        assert_eq!(2, factorial(2));
        
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
