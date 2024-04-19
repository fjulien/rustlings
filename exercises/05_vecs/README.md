# Vectors

Les vecteurs sont l'une des structures de données les plus utilisées en Rust. Dans d'autres langages de
de programmation, on les appellerait simplement des tableaux, mais comme Rust opère à un
un peu plus bas, un tableau en Rust est stocké sur la pile (ce qui signifie qu'il ne peut pas grandir ou rétrécir, et que sa taille doit être connue de tous).
ne peut pas croître ou décroître, et la taille doit être connue au moment de la compilation,
et un vecteur est stocké dans le tas (où ces restrictions ne s'appliquent pas).
Les vecteurs font l'objet d'un chapitre ultérieur du livre, mais nous pensons qu'ils sont suffisamment utiles pour en parler un peu.
mais nous pensons qu'ils sont suffisamment utiles pour en parler un peu plus tôt. Nous parlerons de
l'autre structure de données utile, les cartes de hachage, plus tard.

## Plus d'informations

- [Stocker des listes de valeurs avec des vecteurs](https://doc.rust-lang.org/stable/book/ch08-01-vectors.html)
- [`iter_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.iter_mut)
- [`map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)
