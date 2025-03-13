use std::io;

fn main() {
    println!("Qual seu nome?"); 
    let nome = ler_entrada(); 

    println!("Quantos anos voce tem?");
    let idade: i32 = ler_entrada().trim().parse().expect("Insira um numero valido para idade");

    println!("Qual sua cidade?"); 
    let cidade = ler_entrada();

    println!("\n Informaçoes Cadastradas");
    println!("Nome: {}", nome);
    println!("Idade: {}", idade);
    println!("Cidade: {}", cidade)
}

//Função para ler entrada do usuario 
fn ler_entrada() -> String{
    let mut entrada = String::new(); 
    io::stdin().read_line(& mut entrada).expect("Erro ao ler entrada"); 
    entrada.trim().to_string()

} 
