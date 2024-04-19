// primitive_types2.rs
//
// Remplissez le reste de la ligne où il manque du code ! Pas de conseils, il n'y a pas d'astuces
// astuces, il faut juste s'habituer à les taper :)

fn main() {
    // Caractères (`char`)

    // Notez les guillemets _single_, ils sont différents des guillemets doubles
    // que vous avez pu voir un peu partout.
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabétique!");
    } else if my_first_initial.is_numeric() {
        println!("Numérique!");
    } else {
        println!("Ni alphabétique ni numérique!");
    }

    let your_character = '1';// Terminez cette ligne comme dans l'exemple ! Quel est votre caractère préféré ?
    // Essayez une lettre, un chiffre, un caractère spécial, 
    // un caractère d'une autre langue que la vôtre, essayez un emoji !
    if your_character.is_alphabetic() {
        println!("Alphabétique!");
    } else if your_character.is_numeric() {
        println!("Numérique!");
    } else {
        println!("Ni alphabétique ni numérique!");
    }
}
