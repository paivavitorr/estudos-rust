struct Funcionario {
    id: i32,
    nome: String,
    salario: f32, // Adicionada a vírgula (boa prática)
}

fn main() {
    // 1. Tuplos corrigidos para usar parênteses () e String::from() corrigida
    let func: (i32, String, f32) = (1, String::from("Vitor"), 1992.0);

    // 2. Macros println! corrigidas para usar parênteses ()
    println!("Id: {}", func.0);
    println!("Nome: {}", func.1);
    println!("Salário: {}", func.2);

    // 3. Uso do nome correto da Struct (Funcionario) e String::from já corrigida
    let funcionario: Funcionario = Funcionario {
        id: 1,
        nome: String::from("Vitor"),
        salario: 19553.0,
    };

    println!("ID: {}", funcionario.id);
    println!("Nome: {}", funcionario.nome);
    println!("Salário: {}", funcionario.salario);
}
