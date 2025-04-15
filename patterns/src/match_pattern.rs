pub fn pattern<T>(x:Option<T>) -> Option<T>
where 
    T: std::ops::Add<i32, Output = T>
{
    match x {
        None => None,
        Some(i) => Some(i + 1),    
    }
}