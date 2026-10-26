use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let mut stdout = io::stdout();
    let mut frame = 0;

    // Flag atómica para controlar o loop da animação
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Captura o Ctrl+C usando a crate que o seu Cargo acabou de baixar
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Erro ao configurar o manipulador de Ctrl+C");

    // Esconde o cursor do terminal para uma animação mais limpa
    print!("\x1b[?25l");
    let _ = stdout.flush();

    println!("Iniciando a animação de Modena e Araujo! Pressione Ctrl+C para parar.");
    thread::sleep(Duration::from_secs(1));

    // O loop valida a flag a cada frame
    while running.load(Ordering::SeqCst) {
        // Limpa o terminal e move o cursor para o topo (0, 0)
        print!("\x1b[2J\x1b[H");

        println!("=== CLI ANIMATION ===");
        println!("  Pressione Ctrl+C para sair. Frame: {}\n", frame);

        // Gera 20 linhas de animação baseada em ondas senoidais
        for y in 0..20 {
            let mut line = String::new();

            for x in 0..60 {
                // Função matemática para criar ondas que se movem no tempo (frame)
                let value1 = (((x as f32) * 0.15 + (frame as f32) * 0.15).sin() * 8.0) + 10.0;
                let value2 = (((y as f32) * 0.2 - (frame as f32) * 0.1).cos() * 6.0) + 10.0;

                let dist = ((y as f32 - value1).powi(2) + (x as f32 - value2 * 2.0).powi(2)).sqrt();

                // Escolhe o caractere com base na distância da onda
                if dist < 2.0 {
                    line.push('@');
                } else if dist < 4.0 {
                    line.push('#');
                } else if dist < 6.0 {
                    line.push('*');
                } else if dist < 8.0 {
                    line.push('·');
                } else {
                    line.push(' ');
                }
            }
            // Exibe a linha colorida em tom verde (usando cores ANSI)
            println!("\x1b[32m{}\x1b[0m", line);
        }

        frame += 1;
        let _ = stdout.flush();

        // Controla a velocidade (aproximadamente 30 frames por segundo)
        thread::sleep(Duration::from_millis(33));
    }

    // --- CÓDIGO DE ENCERRAMENTO LIMPO ---
    // Restaura o cursor do terminal (?25h) e limpa o ecrã antes de sair
    print!("\x1b[?25h\x1b[2J\x1b[H");
    println!("Programa encerrado com sucesso. Toda a memória foi libertada!");
    let _ = stdout.flush();
}

