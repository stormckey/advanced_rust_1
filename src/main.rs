struct Buffer<T: std::ops::Add<Output = T>>{
    pub data: Vec<T>,
}

impl<T> Buffer<T>
where
    T: std::ops::Add<Output = T> + Default + Copy,
{
    pub fn new() ->Buffer<T>{
        Buffer { data: Vec::new() } 
    }
    pub fn sum(&self) -> T{
        let mut sum = T::default();
        for item in &self.data{
            sum = sum + *item;
        }
        sum
    }
}

fn compare_string(a:&str, b:&str) -> bool{
    let a_chars = a.chars();
    let b_chars = b.chars();

    for (a_char, b_char) in a_chars.zip(b_chars){
        if a_char != b_char {
            return a_char > b_char;
        }
    }
    a.len() > b.len()
}

fn main(){
    let x = 'a';
    let closure = |x:char|->char {println!("{}",x);x};
    closure(x);
    println!("{}",x);
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_sum_fuc(){
        let mut b = Buffer::new();
        b.data.push(1);
        b.data.push(2);
        assert_eq!(b.sum(),3);
    }

    #[test]
    fn test_compare_string(){
        let s1 = "asdf";
        let s2 = "asdfg";
        let s3 = "asc";
        assert!(!compare_string(&s1,&s2));
        assert!(compare_string(&s2,&s1));
        assert!(compare_string(&s1,&s3));
    }

    #[test]
    fn try_closure(){
        let v0 = vec!['a','b','c','d','e'];
        assert_eq!(vec!['b','c','d','e','f'],v0.iter().map(|&x| (x as u8 +1) as char).collect::<Vec<_>>())
    }

}