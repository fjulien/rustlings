// functions5.rs
//
// Exécutez `rustlings hint functions5` ou utilisez la sous-commande `hint` watch pour un
// pour un indice.

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}
