use std::fmt::{Display, Formatter, Result};
use std::ptr::eq;

type MyArray<T, const N: usize> = [T; N];

struct Wrap<T, const N: usize>(MyArray<T, N>);

impl<T, const N: usize> Display for Wrap<T, N>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let separator = ", ";
        let mut s = "Array -> ".to_string();

        for element in &self.0 {
            s.push_str(&element.to_string());
            if !eq(element, self.0.last().unwrap()) {
                s.push_str(separator);
            }
        }
        write!(f, "{}", s)
    }
}

fn main() {
    let my_array_i: [i32; 0] = [];
    let my_array_f = [1.5, 2.5, 3.5, 4.5, 5.2, 6.75, 8.90, 8.90, 8.90];
    let my_array_c = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'j', 'j'];
    let my_array_s = ["Hello".to_string(), "Newtype!".to_string()];

    let my_wrap = Wrap(my_array_i);
    println!("{}", my_wrap);

    let my_wrap = Wrap(my_array_f);
    println!("{}", my_wrap);

    let my_wrap = Wrap(my_array_c);
    println!("{}", my_wrap);

    let my_wrap = Wrap(my_array_s);
    println!("{}", my_wrap);
}
