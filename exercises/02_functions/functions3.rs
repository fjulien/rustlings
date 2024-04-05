// functions3.rs
//
// Exécutez `rustlings hint functions3` ou utilisez la sous-commande `hint` watch pour un
// pour un indice.

fn main() {
    call_me(5);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
