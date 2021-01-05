use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    vector_examples();
    string_examples();
    hash_map_examples();
    exercise1();
    exercise2();
    exercise3();
}

trait LastRemovable {
    fn remove_last(&mut self);
}

impl<T> LastRemovable for Vec<T> {
    fn remove_last(&mut self) {
        if self.is_empty() {
            return;
        }
        self.remove(self.len() - 1);
    }
}

fn vector_examples() {
    let v: Vec<i32> = Vec::new(); // we need to add a type because we don't insert any value
    println!("{:?}", v);
    // This time we don't need to specify a type because it can be inferred from the values we inserted
    let v = vec![1, 2, 3];
    println!("{:?}", v);

    // Make vector mutable to make it possible to update values
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v.push(9);
    v.remove_last();
    println!("{:?}", v);

    // Reading elements
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    match v.get(5) {
        Some(sixth) => println!("The sixth element is {}", sixth),
        None => println!("There is no sixth element."),
    }

    // The next line would crash
    // let does_not_exist = &v[100];
    // This line wraps the value in an option
    let does_not_exist = v.get(100);

    let first = &v[0];
    // Cannot mutate because we have a immutable borrow in the line above
    // v.push(6);

    println!("The first element is: {}", first);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }

    // Storing different types in a vec
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);
} // <- v goes out of scope and is freed here

fn string_examples() {
    let mut s = String::new();
    println!("{}", s);

    // The next two lines do exactly the same
    let s = "initial contents.".to_string();
    println!("{}", s);
    let s = String::from("initial contents.");
    println!("{}", s);
    // all strings are utf-8 encoded
    let hello = String::from("السلام عليكم");
    println!("{}", hello);
    let hello = String::from("Dobrý den");
    println!("{}", hello);
    let hello = String::from("Hello");
    println!("{}", hello);
    let hello = String::from("שָׁלוֹם");
    println!("{}", hello);
    let hello = String::from("नमस्ते");
    println!("{}", hello);
    let hello = String::from("こんにちは");
    println!("{}", hello);
    let hello = String::from("안녕하세요");
    println!("{}", hello);
    let hello = String::from("你好");
    println!("{}", hello);
    let hello = String::from("Olá");
    println!("{}", hello);
    let hello = String::from("Здравствуйте");
    println!("{}", hello);
    let hello = String::from("Hola");
    println!("{}", hello);

    let mut s = String::from("foo ");
    s.push_str("bar");
    s.push('!');
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);

    // format!
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // unicode length
    let hello = String::from("Hola");
    println!("length of 'Hola': {}", hello.len());

    let hello = String::from("Здравствуйте");
    println!("length of 'Hola': {}", hello.len());
    // The next line will compile but would panic at runtime because the ending index is inside a char.
    // Here each char is stored with 2 bytes
    // let answer = &hello[0..1];
    // println!("the answer is {}", answer);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}

fn hash_map_examples() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);
    println!("{:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // will not compile
    // println!("{}", field_name);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("Have score: {:?}", score);
    if score.is_some() {
        println!("Have score: {:?}", score.unwrap());
    }

    // looping trough a hashmap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // entry function
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "This is so special. This is so great!";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn exercise1() {
    // Given a list of integers, use a vector and return the mean (the average value),
    // median (when sorted, the value in the middle position), and mode
    // (the value that occurs most often; a hash map will be helpful here) of the list.
    let mut values = vec![9, 8, 2, 3, 4, 5, 6, 7, 1, 0, 0];
    let mut average = 0;
    let len = values.len();
    for value in &values {
        average += value;
    }
    let mut float_average = average as f64;
    println!("sum: {}", average);
    println!("len: {}", len);
    average /= len;
    println!("Average is {}", average);
    float_average /= len as f64;
    println!("Float average is {}", float_average);

    values.sort();
    let median = values[len / 2];
    println!("median is {}", median);

    let mut count_map = HashMap::new();
    for value in &values {
        let count = count_map.entry(value).or_insert(0);
        *count += 1;
    }
    let highest = count_map
        .values()
        .find(|&x| count_map.values().all(|y| y <= x));
    println!("Highest: {:?}", highest);
}

fn exercise2() {
    // Convert strings to pig latin. The first consonant of each word is moved to the
    // end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that
    // start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
    // Keep in mind the details about UTF-8 encoding!

    let latin = "This is such a wonderful day, isn't it?";

    let pig_latin: String = latin.split_whitespace().map(transform_word).collect();

    println!("Pig latin: {}", pig_latin);
    println!("Bla");
}

fn transform_word(word: &str) -> String {
    let mut string_at_end = "hay".to_string();
    let mut new_word = String::from("");
    for (index, char) in word.chars().enumerate() {
        if index == 0 {
            if !is_vocal(&char) {
                string_at_end = String::from('-');
                string_at_end.push_str(&*char.to_lowercase().to_string());
                string_at_end.push_str("ay");
            } else {
                new_word.push(char);
            }
        } else {
            new_word.push(char);
        }
    }
    new_word.push_str(&*string_at_end);
    new_word.push_str(" ");
    new_word
}

fn is_vocal(char: &char) -> bool {
    let vocals = ['a', 'e', 'i', 'o', 'u'];
    return vocals.iter().any(|&vocal| vocal == *char);
}

fn exercise3() {
    // Using a hash map and vectors, create a text interface to allow a user to
    // add employee names to a department in a company. For example, “Add Sally to Engineering”
    // or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department
    // or all people in the company by department, sorted alphabetically.
    let inputs = vec![
        String::from("Add Amir to Sales."),
        String::from("Add Timo to Engineering"),
        String::from("Add TimosBrother to Engineering"),
        String::from("Add Lisa to Marketing"),
    ];
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    for input in inputs {
        let item = handle_input(input);
        if let Some(department) = company.get_mut(&item.team) {
            department.push(item.name);
        } else {
            company.insert(item.team, vec![item.name]);
        }
    }
    println!("The company: {:?}", company);
}

struct AddItem {
    team: String,
    name: String,
}

fn handle_input(input: String) -> AddItem {
    let mut name = String::from("");
    let mut team = String::from("");
    for (index, word) in input.split_whitespace().enumerate() {
        if index == 0 && word.replace(".", "").to_lowercase() != "add" {
            panic!("Wrong input!");
        } else if index == 2 && word.to_lowercase() != "to" {
            panic!("Wrong input!");
        } else if index == 1 {
            name = word.parse().unwrap();
        } else if index == 3 {
            team = word.parse().unwrap();
        }
    }
    AddItem { name, team }
}
