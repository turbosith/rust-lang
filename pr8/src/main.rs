use std::collections::HashMap;

fn calculate_statistics(numbers: &Vec<i32>) -> (f64, i32, i32) {
    let sum: i32 = numbers.iter().sum();
    let mean = sum as f64 / numbers.len() as f64;

    let mut sorted_numbers = numbers.clone();
    sorted_numbers.sort();

    let median = if numbers.len() % 2 == 0 {
        let mid = numbers.len() / 2;
        (sorted_numbers[mid - 1] + sorted_numbers[mid]) / 2
    } else {
        sorted_numbers[numbers.len() / 2]
    };

    let mut frequency_map = HashMap::new();
    for num in numbers {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    let mut max_frequency = 0;
    let mut mode = 0;
    for (num, frequency) in frequency_map {
        if frequency > max_frequency {
            max_frequency = frequency;
            mode = *num;
        }
    }

    (mean, median, mode)
}

fn pig_latin(s: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            let first_char = chars.next().unwrap();

            if vowels.contains(&first_char) {
                format!("{}-hay", word)
            } else {
                format!("{}-{}ay", chars.collect::<String>(), first_char)
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 5, 6, 6, 6];
    let (mean, median, mode) = calculate_statistics(&numbers);
    println!("Среднее: {}", mean);
    println!("Медиана: {}", median);
    println!("Мода: {}", mode);


    let s = "first";
    let pig_latin = pig_latin(s);
    println!("{}", pig_latin);


    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Enter a command (add or list):");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Ошибка");
        let tokens: Vec<&str> = input.trim().split_whitespace().collect();

        if tokens.len() < 3 {
            println!("Invalid command");
            continue;
        }

        let name = tokens[1];
        let department = tokens[3];

        match tokens[0] {
            "add" => {
                let employees = company.entry(department.to_string())
                    .or_insert(Vec::new());
                employees.push(name.to_string());
                println!("Added {} to {}", name, department);
            }
            "list" => {
                if department == "company" {
                    let mut departments: Vec<&String> = company.keys().collect();
                    departments.sort();
                    for dept in departments {
                        let employees = company.get(dept).unwrap();
                        println!("{}:", dept);
                        for emp in employees {
                            println!(" - {}", emp);
                        }
                    }
                } else {
                    match company.get(&department.to_string()) {
                        Some(employees) => {
                            println!("{}:", department);
                            for emp in employees {
                                println!(" - {}", emp);
                            }
                        }
                        None => println!("Департамент не найден"),
                    }
                }
            }
            _ => println!("Ошибочная команда"),
        }
    }
}


