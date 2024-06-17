pub struct Solution;

//Reverse a vector of chars
impl Solution{
    pub fn reverse_string(s: &mut Vec<char>){
        Self::solve(s, 0, s.len() - 1);
    }
    fn solve(s: &mut Vec<char>, start: usize, end: usize){
        if start >= end {
            return;
        }
        Self::swap(s, start, end);
        Self::solve(s, start + 1, end - 1);
    }
    fn swap(s: &mut Vec<char>, i: usize, j:usize){
        let temp = s[i];
        s[i] = s[j];
        s[j] = temp;
    }
    
}

fn main(){
    let mut s = vec!['h','e','l','l','o'];
    Solution::reverse_string(&mut s);
    println!("{:?}", s);
}