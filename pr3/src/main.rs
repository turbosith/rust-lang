use std::io;

fn main() {
    loop{
    println!("Выберете, что вам нужно сделать:\n
    1-Перевести температору из градусов Фаренгейта в Цельсия\n
    2-Получить число Фибоначи под номером n\n
    3-Вывести песню");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)//ввод выбора действия
        .expect("Ошибка чтения");

    let choice: u32 = choice.trim().parse()
        .expect("Введите корректное число");
    if choice == 1 {
        temp();
    }
    if choice == 2 {
        fib();
    }
    if choice == 3 {
        song();
    }
}

fn fib() {//функция вычисления числа Фибоначи по номеру
    println!("Введите число n, номер числа Фибоначи:");//нахождение числа Фибоначи

    let mut n = String::new();

    io::stdin().read_line(&mut n)
        .expect("Ошибка чтения");

    let n: u32 = n.trim().parse()
        .expect("Введите корректное число");

    let mut fib = (0, 1);

    for _ in 0..n - 1 {
        fib = (fib.1, fib.0 + fib.1);
    }
    let result = fib.1 + 1;
    println!(" Число Фибоначи под номером {n} - это {result}");
}

fn temp() {//функция перевода градусов по Фаренгейту в градусы Цельсия
    println!("Введите температуру в градусах по Фаренгейту:");//перевод температуры
    // из Фаренгейта в Цельсия

    let mut fahrenheit = String::new();
    io::stdin().read_line(&mut fahrenheit)
        .expect("Ошибка чтения");

    let fahrenheit: f32 = fahrenheit.trim().parse()
        .expect("Введите корректное число");

    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;

    println!("{fahrenheit}°F равно {celsius}°C");
}

fn song() {
    let days = ["first", "second", "third",
        "fourth", "fifth", "sixth", "seventh",
        "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts = ["a partridge in a pear tree",
        "two turtle doves", "three French hens",
        "four calling birds", "five golden rings",
        "six geese a-laying", "seven swans a-swimming",
        "eight maids a-milking", "nine ladies dancing",
        "ten lords a-leaping", "eleven pipers piping",
        "twelve drummers drumming"];

    for i in 0..12 {
        println!("On the {} day of Christmas, my true love gave to me:", days[i]);

        for j in (0..i + 1).rev() {
            if j == 0 && i != 0 {
                print!("and ");
            }
            println!("{}", gifts[j]);
        }

        println!("");
    }}
}

