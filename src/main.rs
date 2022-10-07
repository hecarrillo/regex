use std::io;
use std::collections::HashSet; 

fn main() {
  let language_input_type: String = read_language_input_type();
  let mut user_language: HashSet<String> = HashSet::new();
  // if the user input type is 1, read the language from range, else from a list
  if language_input_type == "1" {
    user_language = read_language_from_range();
  } else if language_input_type == "2" {
    user_language = read_language_from_list();
  }
  // print the language
  println!("Your language is: {:?}", user_language);
}

fn read_language_input_type() -> String {
  let stdin = io::stdin();

  println!("Seleccione la forma de ingresar el alfabeto:");
  println!("1) Por rango"); 
  println!("2) De manera individual"); 
  
  let mut input_form = String::new();
  stdin.read_line(&mut input_form);
  
  return input_form.trim().to_string();
}

fn read_language_from_list () -> HashSet<String> {
  let stdin = io::stdin();
  let mut user_language: HashSet<String> = HashSet::new();
  
  println!("Ingrese el alfabeto de la gramática, separado por espacios:");
  let mut input_language = String::new();
  stdin.read_line(&mut input_language);
  
  for symbol in input_language.split_whitespace() {
    user_language.insert(symbol.to_string());
  }
  
  return user_language;
}

fn read_language_from_range () -> HashSet<String> {
  println!("Ingrese el rango de caracteres del alfabeto de la siguiente forma: <primer_caracter>-<ultimo_caracter>");
  let mut user_input_range = String::new();
  let stdin = io::stdin();
  stdin.read_line(&mut user_input_range);

  let first_language_char_as_int: i32 = user_input_range.chars().nth(0).unwrap() as i32;
  let last_language_char_as_int: i32 = user_input_range.chars().nth(2).unwrap() as i32;
  
  let mut user_language: HashSet<String> = HashSet::new();
  for i in first_language_char_as_int..last_language_char_as_int {
    user_language.insert((i as u8 as char).to_string());
  }
  return user_language;
}
