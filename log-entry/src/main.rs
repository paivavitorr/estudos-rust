
#[derive(Debug)]
struct LogEntry {
    ip: String, 
    status_code: u16,
    endpoint: String,
}

fn contar_total_logs(logs: &[LogEntry]) -> usize {
    logs.len()
}
// Função 2: Retorna um vetor com os endpoints que deram o status code desejado
fn buscar_endpoints_por_status(logs: &[LogEntry], status_alvo: u16) -> Vec<&str> {
    logs.iter()
        .filter(|log| log.status_code == status_alvo)
        .map(|log| log.endpoint.as_str())
        .collect()
}

fn main() {
    let logs = vec![
        LogEntry {
            ip: "192.168.1.1".into(),
            status_code: 200,
            endpoint: "/api/users".into(),
        },
        LogEntry {
            ip: "192.168.1.15".into(),
            status_code: 404,
            endpoint: "/api/admin".into(),
        },
        LogEntry {
            ip: "10.0.0.5".into(),
            status_code: 500,
            endpoint: "/api/login".into(),
        },
    ];
    let erros_404 = buscar_endpoints_por_status(&logs, 404);
    println!("Endpoints com erro 404: {:?}", erros_404);

    // 3. Chamando a função e imprimindo o resultado
    let total = contar_total_logs(&logs);
    println!("Total de logs: {}", total);

    // 4. Chamando a função de busca e imprimindo o resultado
    let endpoints_200 = buscar_endpoints_por_status(&logs, 200);
    println!("Endpoints com status 200: {:?}", endpoints_200);
}