fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2: Vec<i32> = v![1, 2, 3];

    assert_eq!(vec1, vec2);
    println!("Success!")
}

#[macro_export]
macro_rules! v {
    ($($x:expr),*) => {
        {
            let mut temp = Vec::new();
            $( temp.push($x); )*
            temp
        }
    };
}
