// from_into.rs
//
// The From trait is used for value-to-value conversions. If From is implemented
// correctly for a type, the Into trait should work conversely. You can read
// more about it at https://doc.rust-lang.org/std/convert/trait.From.html
//
// Execute `rustlings hint from_into` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// We implement the Default trait to use it as a fallback
// when the provided string is not convertible into a Person object
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// Your task is to complete this implementation in order for the line `let p =
// Person::from("Mark,20")` to compile Please note that you'll need to parse the
// age component into a `usize` with something like `"4".parse::<usize>()`. The
// outcome of this needs to be handled appropriately.
//
// Steps:
// 1. If the length of the provided string is 0, then return the default of
//    Person.
// 2. Split the given string on the commas present in it.
// 3. Extract the first element from the split operation and use it as the name.
// 4. If the name is empty, then return the default of Person.
// 5. Extract the other element from the split operation and parse it into a
//    `usize` as the age.
// If while parsing the age, something goes wrong, then return the default of
// Person Otherwise, then return an instantiated Person object with the results


impl From<&str> for Person {
    fn from(s: &str) -> Person {
        // 如果字符串为空，返回默认的 Person
        if s.is_empty() {
            return Person::default();
        }

        // 使用 split_once 分割字符串
        let (name, age_str) = s.split_once(',').unwrap_or(("", ""));

        // 如果名字为空，返回默认的 Person
        if name.trim().is_empty() {
            return Person::default();
        }

        // 尝试解析年龄，如果失败则返回默认的 Person
        let age = match age_str.trim().parse::<usize>() {
            Ok(x) if x < 150 && x > 0 => x,
            _ => return Person::default()
        };

        // 检查年龄是否在合理范围内
        if age > 150 {
            return Person::default();
        }

        // 返回解析后的 Person 对象
        Person {
            name: name.trim().to_string(),
            age,
        }
    }
}

fn main() {
    // Use the `from` function
    let p1 = Person::from("Mark,20");
    // Since From is implemented for Person, we should be able to use Into
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        // Test that the default person is 30 year old John
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        // Test that John is returned when bad string is provided
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        // Test that "Mark,20" works
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_bad_age() {
        // Test that "Mark,twenty" will return the default person due to an
        // error in parsing age
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
