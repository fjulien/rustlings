// functions2.rs
//
// Exécutez `rustlings hint functions2` ou utilisez la sous-commande `hint` watch pour un
// pour un indice.

fn main() {
    call_me(3);
}

fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
