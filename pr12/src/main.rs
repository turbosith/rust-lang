use std::env;
use std::fs;
fn main() {

    let args: Vec<String> = env::args().collect();


    let config = Config::new(&args);
    //let args: Vec<String> = env::args().collect();//добавление структуры для группировки пути файла и строки

    //let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");
    // let args: Vec<String> = env::args().collect();

    //let (query, file_path) = parse_config(&args);//убираем парсер в отдельную функцию
    //let args: Vec<String> = env::args().collect();//получение строки из командной строки
    //let query = &args[1];//первый аргумент, что будем искать
   // let file_path = &args[2];//второй аргумент, где будем искать

  //  println!("Searching for {}", query);
    //println!("In file {}", file_path);
   // println!("In file {}", file_path);

    //let contents = fs::read_to_string(file_path)//чтение файла
      //  .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}
struct Config {//структура для поиска
    query: String,
    file_path: String,
}
impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}