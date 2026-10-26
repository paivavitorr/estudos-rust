use std::io;

fn idade() {
    println!("Digite o seu ano de nascimento:");

    let mut ano_nascimento = String::new();
    io::stdin()
        .read_line(&mut ano_nascimento)
        .expect("Failed to read input");

    let ano_atual: i16 = 2026;
    let idade: i16 = ano_atual - ano_nascimento.trim().parse::<i16>().expect("Please enter a valid number");

    println!("Você tem {} anos.", idade);

    println!("Você quer saber se você é maior de idade: (Digite 'sim' ou 'não')");
    let mut resposta = String::new();
    io::stdin()
        .read_line(&mut resposta)
        .expect("Failed to read input");    
    let resposta = resposta.trim().to_lowercase();  
    let maior_de_idade = idade >= 18;
    
    if resposta == "sim" {
        if maior_de_idade {
            println!("Você é maior de idade.");
        } else {
            println!("Você não é maior de idade.");
        }
    }
}

fn main() {
    agenda_telefonica();
    // Se quiser testar a função idade, basta descomentar a linha abaixo:
    // idade();
}

fn agenda_telefonica() {
    let msg: &str = "Falha ao ler a entrada";

    println!("Bem vindo a sua Agenda Telefonica!");
    let mut agenda: Vec<(String, String)> = Vec::new(); 

    loop {
        println!("\nEscolha uma opção abaixo:");
        println!("1 - Adicionar contato");
        println!("2 - Listar contatos");
        println!("3 - Editar contato");
        println!("4 - Deletar contato");
        println!("5 - Sair");

        let mut opcao = String::new();
        io::stdin()
            .read_line(&mut opcao)
            .expect(msg);

        match opcao.trim() { 
            "1" => {
                println!("Digite o nome do contato:");
                let mut nome = String::new();
                io::stdin()
                    .read_line(&mut nome)
                    .expect(msg);
                
                println!("Digite o telefone do contato:");
                let mut telefone = String::new();
                io::stdin()
                    .read_line(&mut telefone)
                    .expect(msg);

                // Limpando os espaços e quebras de linha (\n)
                let nome = nome.trim().to_string();
                let telefone = telefone.trim().replace("-", "").replace(" ", "");

                agenda.push((nome, telefone));
                println!("Contato adicionado com sucesso!");
            },
            "2" => {
                println!("\n--- Lista de Contatos ---");
                if agenda.is_empty() {
                    println!("Nenhum contato cadastrado.");
                } else {
                    for (i, (nome, telefone)) in agenda.iter().enumerate() {
                        println!("{} - {}: {}", i + 1, nome, telefone);
                    }
                }
                println!("-------------------------");
            },
            "3" => {
                println!("Digite o número do contato que deseja editar:");
                let mut index = String::new();
                io::stdin()
                    .read_line(&mut index)
                    .expect(msg);
                
                let index: usize = match index.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Por favor, insira um número válido.");
                        continue;
                    }
                };

                if index > 0 && index <= agenda.len() {
                    println!("Digite o novo nome do contato:");
                    let mut nome = String::new();
                    io::stdin()
                        .read_line(&mut nome)
                        .expect(msg);
                    
                    println!("Digite o novo telefone do contato:");
                    let mut telefone = String::new();
                    io::stdin()
                        .read_line(&mut telefone)
                        .expect(msg);

                    let nome = nome.trim().to_string();
                    let telefone = telefone.trim().replace("-", "").replace(" ", "");
                    
                    agenda[index - 1] = (nome, telefone);
                    println!("Contato atualizado com sucesso!");
                } else {
                    println!("Contato não encontrado.");
                }
            },
            "4" => {
                println!("Digite o número do contato que deseja deletar:");
                let mut index = String::new();
                io::stdin()
                    .read_line(&mut index)
                    .expect(msg);
                
                let index: usize = match index.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Por favor, insira um número válido.");
                        continue;
                    }
                };

                if index > 0 && index <= agenda.len() {
                    agenda.remove(index - 1);
                    println!("Contato deletado com sucesso.");
                } else {
                    println!("Contato não encontrado.");
                }
            },
            "5" => {
                println!("Saindo da agenda...");
                break;
            },
            _ => println!("Opção inválida! Por favor, tente novamente.")
        }
    }   
}