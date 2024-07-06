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

#[cfg(test)]
mod tests {
    #[test]
    fn newtype_i32() {
        let my_array = [1, 2, 3];
        let my_wrap = super::Wrap(my_array);
        assert_eq!(my_wrap.to_string(), "Array -> 1, 2, 3");
    }

    #[test]
    fn newtype_f64() {
        let my_array = [1.5, 2.5, 3.5, 4.5, 5.2, 6.75, 8.90, 8.90, 8.90];
        let my_wrap = super::Wrap(my_array);
        assert_eq!(
            my_wrap.to_string(),
            "Array -> 1.5, 2.5, 3.5, 4.5, 5.2, 6.75, 8.9, 8.9, 8.9"
        );
    }

    #[test]
    fn newtype_char() {
        let my_array = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'j', 'j'];
        let my_wrap = super::Wrap(my_array);
        assert_eq!(
            my_wrap.to_string(),
            "Array -> a, b, c, d, e, f, g, h, i, j, j, j"
        );
    }

    #[test]
    fn newtype_string() {
        let my_array = ["Hello".to_string(), "Newtype!".to_string()];
        let my_wrap = super::Wrap(my_array);
        assert_eq!(my_wrap.to_string(), "Array -> Hello, Newtype!");
    }
}
