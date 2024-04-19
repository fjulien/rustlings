// vecs1.rs
//
// Votre tâche est de créer un `Vec` qui contient exactement les mêmes éléments que dans le tableau `a`.
// tableau `a`.
//
// Faites-moi compiler et passer le test !
//
// Exécutez `rustlings hint vecs1` ou utilisez la sous-commande `hint` watch pour obtenir un indice.

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    let v :Vec<i32> = Vec::from(a);// TODO: déclarez votre vecteur ici avec la macro pour les vecteurs

    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}
