// Funções de acesso a dados para solicitações
pub async fn get_request_data(request_id: &str) -> String {
    // Simule uma operação de banco de dados
    format!("Dados da solicitação: {}", request_id)
}
