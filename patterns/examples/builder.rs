#![deny(warnings)]

fn main() {
    println!("Hello, builder for tests!");
}

pub fn sum_all(values: &[i32]) -> i32 {
    let mut result = 0;
    for v in values {
        result += v
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[must_use]
    struct SumAllTest {
        expected: i32,
        values: Vec<i32>,
    }

    impl SumAllTest {
        fn expect(value: i32) -> Self {
            Self {
                expected: value,
                values: vec![],
            }
        }

        fn add_value(mut self, value: i32) -> Self {
            self.values.push(value);
            self
        }

        fn run(self) {
            let actual = sum_all(&self.values);
            assert_eq!(self.expected, actual);
        }
    }

    #[test]
    fn test_sum_all_with_no_values() {
        SumAllTest::expect(0).run();
    }

    #[test]
    fn test_sum_all_with_one_value() {
        SumAllTest::expect(5).add_value(5).run();
    }

    #[test]
    fn test_sum_all_with_many_values() {
        SumAllTest::expect(8)
            .add_value(3)
            .add_value(4)
            .add_value(1)
            .run();
    }
}
