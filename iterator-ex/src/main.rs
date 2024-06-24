struct A {
    name: String,
}

impl A {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

fn main() {
    let vec = vec![1, 2, 3, 4];
    let arr = [1, 2, 3, 4];
    let slice = vec.as_slice();
    let slice_arr = arr.as_slice();

    let x = "abc".to_string();
    let a = A::new("Hello");
    let b = A::new(String::from("Yolo"));
    let strng = String::from("Hello");

    let strng_1: String = strng.into(); // No re-allocation See impl<T> From<T> for T
    let strng_2: String = strng_1.to_string(); // Reallocation, see impl ToString for String
    let slice = strng_2.as_str(); // Same data
}
