use rand::Rng; // Para generar números aleatorios
use rand::seq::SliceRandom; // Para mezclar elementos de un slice

fn main() {
    let letras = ['a', 'e', 'i', 'o', 'u', 'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'y', 'z'];
    let longitud_palabra = rand::thread_rng().gen_range(3..8); // Genera una longitud de palabra aleatoria entre 3 y 7

    let palabra: String = (0..longitud_palabra)
        .map(|_| *letras.choose(&mut rand::thread_rng()).unwrap())
        .collect();

    println!("Palabra generada: {}", palabra);
}
