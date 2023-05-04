struct Color(i32, i32, i32);

struct Point(i32, i32, i32);
#[derive(Debug)]
struct Rectangle {//структура Прямоугольник
    width: u32,
    height: u32,
}

impl Rectangle {//описание метода вычисления площади
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let mut user1 = User {//создания атрибута пользователя
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
  let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

struct User {//структура пользователя
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {//создание юзера
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
