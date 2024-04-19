// move_semantics6.rs
//
// Vous ne pouvez rien changer, sauf ajouter ou supprimer des références.
//
// Exécutez `rustlings hint move_semantics6` ou utilisez la sous-commande watch `hint` pour un indice.
// pour un indice.

// I AM NOT DONE

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// Ne doit pas s'approprier la propriété
fn get_char(mut data: &String) -> char {
    data.chars().last().unwrap()
}

// Doit s'approprier le projet
fn string_uppercase(data: String) {
    let data = data.to_uppercase();
    
    println!("{}", data);
}
