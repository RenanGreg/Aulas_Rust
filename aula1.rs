
fn main() {
    let numeros = vec![10, 20, 30, 40, 50];

    for numero in &numeros {
        println!("{}", numero);
    }
}
























fn main() {
    let mut vetor = Vec::new();

    vetor.push(5);
    vetor.push(10);
    vetor.push(15);

    println!("{:?}", vetor); // Deve imprimir [5, 10, 15]
}






















fn main() {
    let frutas = vec!["maçã", "banana", "laranja"];

    let primeira = frutas[0];
    let ultima = frutas[frutas.len() - 1];

    println!("Primeira fruta: {}", primeira);
    println!("Última fruta: {}", ultima);
}





















fn main() {
    let numeros = vec![1, 2, 3, 4, 5];
    let tamanho = numeros.len();

    println!("O vetor tem {} elementos.", tamanho);
}























fn main() {
    let mut vetor = vec![10, 20, 30];
    vetor.pop(); // Remove o último elemento

    println!("{:?}", vetor); // Deve imprimir [10, 20]
}

























fn main() {
    let numeros = vec![1, 2, 3, 4, 5];
    let soma: i32 = numeros.iter().sum();

    println!("A soma dos elementos é: {}", soma);
}




fn main() {
    let frutas = vec!["maçã", "banana", "laranja"];
    let procura = "banana";

    if frutas.contains(&procura) {
        println!("{} está no vetor.", procura);
    } else {
        println!("{} não está no vetor.", procura);
    }
}







fn main() {
    let vetor = vec![0; 5]; // Cria um vetor com 5 elementos, todos iguais a 0

    println!("{:?}", vetor); // Deve imprimir [0, 0, 0, 0, 0]
}

























fn main() {
    let frutas = vec!["maçã", "banana", "laranja"];

    for (indice, fruta) in frutas.iter().enumerate() {
        println!("Índice: {}, Fruta: {}", indice, fruta);
    }
}





















fn main() {
    let vetor1 = vec![1, 2, 3];
    let vetor2 = vec![4, 5, 6];
    let combinado = [&vetor1[..], &vetor2[..]].concat();

    println!("{:?}", combinado); // Deve imprimir [1, 2, 3, 4, 5, 6]
}


