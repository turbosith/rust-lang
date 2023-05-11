pub fn add_two(a: i32) -> i32 {//функция добавления 2
    a + 3
}

pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}
fn prints_and_returns_10(a: i32) -> i32 {//функция печатающая число и возвращающая 10
    println!("I got the value {}", a);
    10
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn exploration() {//упрощенная запись теста it_works
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {//возвращение ошибки
        panic!("Make this test fail");
    }

    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {//создание экземпляра класса Rectangle
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {//создание экземпляра класса Rectangle
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));//проверка логического условия
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    pub fn greeting(name: &str) -> String {
        String::from("Hello!")
    }


    #[test]
    fn greeting_contains_name() {//проверка того, что возврат
        // метода содержит имя с возвратом сообщения ошибки
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    #[test]
    fn this_test_will_pass() {//тест пройдет, потому что метод всегда возвращает 10
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {//тест не пройдет, потому что метод всегда возвращает 10
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}

#[derive(Debug)]
struct Rectangle {
    //описание структуры
    width: u32,
    height: u32,
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {//метод для тестирования
        self.width > other.width && self.height > other.height
    }
}