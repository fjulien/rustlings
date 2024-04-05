// intro1.rs
//
// A propos de ce truc "I AM NOT DONE" :
// Nous vous encourageons parfois à continuer à faire des essais pour un exercice donné, même
// après l'avoir résolu. Si tout fonctionne et que vous vous sentez
// prêt à passer à l'exercice suivant, supprimez le commentaire "I AM NOT DONE" ci-dessous.
//
// Si vous utilisez `rustlings watch` : Le fichier d'exercice sera
// Le fichier de l'exercice sera rechargé lorsque vous modifierez l'une des lignes ci-dessous ! Essayez d'ajouter une ligne `println!`
// ou essayez de changer ce qu'il affiche dans votre terminal. Essayez de supprimer un
// point-virgule et voyez ce qui se passe !
//
// Exécutez `rustlings hint intro1` ou utilisez la sous-commande `hint` watch pour un
// pour un indice.

fn main() {
    println!("Hello and");
    println!(r#"       welcome to...                      "#);
    println!(r#"                 _   _ _                  "#);
    println!(r#"  _ __ _   _ ___| |_| (_)_ __   __ _ ___  "#);
    println!(r#" | '__| | | / __| __| | | '_ \ / _` / __| "#);
    println!(r#" | |  | |_| \__ \ |_| | | | | | (_| \__ \ "#);
    println!(r#" |_|   \__,_|___/\__|_|_|_| |_|\__, |___/ "#);
    println!(r#"                               |___/      "#);
    println!();
    println!("This exercise compiles successfully. The remaining exercises contain a compiler");
    println!("or logic error. The central concept behind Rustlings is to fix these errors and");
    println!("solve the exercises. Good luck!");
    println!();
    println!("The source for this exercise is in `exercises/00_intro/intro1.rs`. Have a look!");
    println!(
        "Going forward, the source of the exercises will always be in the success/failure output."
    );
    println!();
    println!(
        "If you want to use rust-analyzer, Rust's LSP implementation, make sure your editor is set"
    );
    println!("up, and then run `rustlings lsp` before continuing.")
}
