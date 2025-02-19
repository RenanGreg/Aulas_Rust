fn main() {
    let frutas = vec!["maçã", "banana", "laranja"];

    for (indice, fruta) in frutas.iter().enumerate() {
        println!("Índice: {}, Fruta: {}", indice, fruta);
    }
}
