use std::io;
use std::collections::HashSet; 
use regex::Regex;
use rand::Rng;

fn main() {
  let user_alphabet = read_user_defined_alphabet();
  // print the alphabet
  println!("El alfabeto es: {:?}", user_alphabet);
  // read two strings and check if they are valid
  println!("____ OPERACIONES CON CADENAS: SUFIJOS, PREFIJOS, SUBCADENAS Y SUBSECUENCIAS ____");
  let string1 = read_valid_string(&user_alphabet);
  let string2 = read_valid_string(&user_alphabet);
  println!(" {} ", check_string_relationship(&string1,&string2));
  
  println!("____ LENGUAJES REGULARES ____");
  let data: Vec<u32> = data_language();
  let language1: HashSet<String> = generate_language(&data,&user_alphabet);
  let language2: HashSet<String> = generate_language(&data,&user_alphabet);
  println!(" Los nuevos lenguajes generados son: {:?} y {:?} ", language1, language2);
  println!(" La resta de los 2 lenguajes es: {:?}", substraction(language1, language2));

  // Rise the alphabet to the power of a user defined number
  println!("\n____ OPERACIONES CON CADENAS: POTENCIA DE UN ALFABETO ____");
  let power: usize= read_user_defined_power();
  let alphabet_rose_to_n = rise_alphabet_to_n(&user_alphabet, power);
  println!(" El alfabeto a la potencia n es {:?}", alphabet_rose_to_n);

  // Read a string and check if it has the vowels in it in order using regex
  println!("\n____ EXPRESIONES REGULARES: VOCAL CONSECUTIVA ____");
  // Regex to check if a string has all the vowels in it in order
  let vowels_regex = Regex::new(r"^([^aeiou]*a[^eiou]*e[^iou]*i[^ou]*o[^u]*u[a-z]*)$").unwrap();
  // Read a string until it matches the regex
  let mut string_with_vowels_in_order = String::new();
  while !vowels_regex.is_match(&string_with_vowels_in_order) {
    println!("Ingrese una cadena que contenga las vocales en orden:");
    io::stdin().read_line(&mut string_with_vowels_in_order).expect("Failed to read line");
    string_with_vowels_in_order = string_with_vowels_in_order.trim().to_string();
  }
}

fn substraction(language1: HashSet<String>, language2: HashSet<String>) -> HashSet<String>{
  let mut new_language: HashSet<String> = language1;
  for element in language2.iter(){
    new_language.remove(element);
  }
  return new_language;
}

fn generate_language(data: &Vec<u32>, user_alphabet: &HashSet<String>) -> HashSet<String>{
  let mut rng = rand::thread_rng();
  let mut new_language: HashSet<String> = HashSet::new();
  let mut symbol_count: u32 = 0;
  while new_language.len() < data[0].clone() as usize{
    let mut string = String::new();
    for i in 0..data[1]{
      // insert a random symbol from the alphabet
      string.push_str(user_alphabet.iter().nth(rng.gen_range(0, user_alphabet.len())).unwrap());
    }
    new_language.insert(string);
  }
  return new_language;
}

fn data_language() -> Vec<u32> {
  let mut elements_and_lenght: Vec<u32> = Vec::new();
  let mut input = String::new();
  println!("Ingresa el numero de elementos y la longitud de los lenguajes a generar separados por un espacio: ");
  io::stdin().read_line(&mut input).expect("Failed to read line");
  for symbol in input.split_whitespace() {
    elements_and_lenght.push(symbol.parse::<u32>().unwrap());
  }
  return elements_and_lenght;
}

fn read_user_defined_power() -> usize {
  println!("Ingrese un numero entero positivo:");
  let mut power = String::new();
  io::stdin().read_line(&mut power).expect("Failed to read line");
  let power: usize = power.trim().parse().expect("Please type a number!");
  return power;
}

fn rise_alphabet_to_n(alphabet: &HashSet<String>, n: usize) -> HashSet<String> {
  let mut combinations: HashSet<String> = HashSet::new();
  if n == 0 {
    combinations.insert("".to_string());
    return combinations;
  }
  for character in alphabet {
    let previous_combinations = rise_alphabet_to_n(alphabet, n - 1);
    // create a vector to store all new combinations 
    let mut new_combinations_vector: Vec<String> = Vec::new();
    for combination in previous_combinations {
      let mut new_combination = combination.clone();
      new_combination.push_str(character);
      new_combinations_vector.push(new_combination);
    }
    // add all new combinations to the set
    for combination in new_combinations_vector {
      combinations.insert(combination);
    }
  }
  return combinations;
}

fn check_string_relationship(string1: &String, string2: &String) -> String {
  let mut combination : String = String::new();
  while combination == ""{
    if string2.contains(string1){
      combination = is_prefix_suffix_or_substring(&string1, &string2);
    } else {
      combination = is_subsequence(&string1, string2.to_string()).to_string();
  }
  }
  return combination.to_string();
}

fn is_prefix_suffix_or_substring(string1: &String, string2: &String) -> String{

  if string2.len() == string1.len(){
    return "La cadena 1 es un sufijo y prefijo impropio de la cadena 2".to_string();
  }
  if string2.starts_with(string1){
    return "La cadena 1 es un prefijo propio de la cadena 2".to_string();
  } else if string2.ends_with(string1){
    return "La cadena 1 es un sufijo propio de la cadena 2".to_string();
  }
  return "La cadena 1 es una subcadena de la cadena 2".to_string();
  
}

fn is_subsequence(string1: &String, string2 : String) -> String {
  let  string1 = string1.clone();
  let  string2 = string2.clone();
  let  string1_chars: Vec<char> = string1.chars().collect();
  let  string2_chars: Vec<char> = string2.chars().collect();
  let mut i = 0;
  let mut j = 0;
  while i < string1_chars.len() && j < string2_chars.len() {
    if string1_chars[i] == string2_chars[j] {
      i += 1;
      j += 1;
    } else {
      j += 1;
    }
  }
  if i == string1_chars.len() {
    return "La cadena 1 es una subsecuencia de la cadena 2".to_string();
  }
  return "La cadena 1 no es una subsecuencia de la cadena 2".to_string();
}

fn read_valid_string(alphabet: &HashSet<String>) -> String {
  println!("Ingrese una cadena valida:");
  let mut valid_string: String = String::new();
  while valid_string == "" {
    let user_input_string = read_string();
    if is_valid_string(&user_input_string, &alphabet){
      valid_string = user_input_string.clone();
    } else {
      println!("La cadena ingresada no es valida, intente nuevamente:");
    }
  }
  return valid_string;
}

fn read_user_defined_alphabet() -> HashSet<String> {
  let alphabet_input_type: String = read_alphabet_input_type();
  let mut user_alphabet: HashSet<String> = HashSet::new();
  // if the user input type is 1, read the alphabet from range, else from a list
  while user_alphabet.len() == 0 {
    if alphabet_input_type == "1" {
      user_alphabet = read_alphabet_from_range();
    } else if alphabet_input_type == "2" {
      user_alphabet = read_alphabet_from_list();
    }
    if user_alphabet.len() == 0 {
      println!("El alfabeto ingresado no es valido, intente nuevamente:");
    }
  }
  return user_alphabet;
}

fn read_string() -> String {
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Failed to read line");
  return input.trim().to_string();
}

fn read_alphabet_input_type() -> String {
  println!("Seleccione la forma de ingresar el alfabeto:");
  println!("1) Por rango"); 
  println!("2) De manera individual"); 
  
  return read_string();
}

fn read_alphabet_from_list () -> HashSet<String> {
  let mut user_alphabet: HashSet<String> = HashSet::new();
  
  println!("Ingrese el alfabeto de la gramática, separado por espacios:");
  let input_alphabet: String = read_string();
  
  for symbol in input_alphabet.split_whitespace() {
    user_alphabet.insert(symbol.to_string());
  }
  
  return user_alphabet;
}

fn read_alphabet_from_range () -> HashSet<String> {
  println!("Ingrese el rango de caracteres del alfabeto de la siguiente forma: <primer_caracter>-<ultimo_caracter>");
  let user_input_range = read_string();

  let first_alphabet_char_as_int: i32 = user_input_range.chars().nth(0).unwrap() as i32;
  let last_alphabet_char_as_int: i32 = user_input_range.chars().nth(2).unwrap() as i32;
  
  let mut user_alphabet: HashSet<String> = HashSet::new();
  for i in first_alphabet_char_as_int..=last_alphabet_char_as_int {
    user_alphabet.insert((i as u8 as char).to_string());
  }
  return user_alphabet;
}

fn is_valid_string (string: &String, alphabet: &HashSet<String>) -> bool {
  if string.len() == 0 {
    return true;
  }
  let mut current_symbol: String = String::new();
  let mut final_valid_string: String = String::new();
  for character in string.chars() {
    current_symbol.push(character);
    if alphabet.contains(&current_symbol) {
      final_valid_string.push_str(&current_symbol);
      current_symbol = String::new();
    }
  }
  return string.len() == final_valid_string.len();
}