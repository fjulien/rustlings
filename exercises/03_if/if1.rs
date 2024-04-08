// if1.rs
//
// Exécutez `rustlings hint if1` ou utilisez la sous-commande de surveillance `hint` pour obtenir une indication.

pub fn bigger(a: i32, b: i32) -> i32 {
    // Complétez cette fonction pour renvoyer le plus grand nombre !
    // Si les deux nombres sont égaux, n'importe lequel d'entre eux peut être renvoyé.
    // Ne pas utiliser :
    // - un autre appel de fonction
    // - des variables supplémentaires

    if a > b {
        a
    } else {
        b
    }
}

// Ne vous en préoccupez pas pour l'instant :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
