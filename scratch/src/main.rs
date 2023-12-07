fn main () {
    let num = "4206".to_string();
    let res = Solution::largest_odd_number(num);
    println!("res = {}", res);
}


struct Solution;

impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        for (i, c) in num.chars().rev().enumerate() {
        
            if c as i32 % 2 == 1 {
                println!("i = {}, c = {}", i, c);
                return num[0..(num.len()-i)].to_string()
            }
        }
        return "".to_string()
    }
}