use std::io;
use std::collections::HashSet; 

fn main() {
  let user_alphabet: HashSet<String> = read_user_defined_alphabet();
  // print the alphabet
  println!("El alfabeto es: {:?}", user_alphabet);
  // read two strings and check if they are valid
  let string1 = read_valid_string(&user_alphabet);
  let string2 = read_valid_string(&user_alphabet);
  // print the strings
  println!("La primera cadena es: {}", string1);
  println!("La segunda cadena es: {}", string2);
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
  println!("Please enter a string: ");
  io::stdin().read_line(&mut input).expect("Failed to read line");
  return input.trim().to_string();
}

fn read_alphabet_input_type() -> String {
  let stdin = io::stdin();

  println!("Seleccione la forma de ingresar el alfabeto:");
  println!("1) Por rango"); 
  println!("2) De manera individual"); 
  
  let mut input_form = String::new();
  stdin.read_line(&mut input_form);
  
  return input_form.trim().to_string();
}

fn read_alphabet_from_list () -> HashSet<String> {
  let stdin = io::stdin();
  let mut user_alphabet: HashSet<String> = HashSet::new();
  
  println!("Ingrese el alfabeto de la gramática, separado por espacios:");
  let mut input_alphabet = String::new();
  stdin.read_line(&mut input_alphabet);
  
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