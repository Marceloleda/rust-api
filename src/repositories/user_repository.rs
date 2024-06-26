// Funções de acesso a dados para usuários
pub async fn get_user_data(user_id: &str) -> String {
    // Simule uma operação de banco de dados
    format!("Dados do usuário: {}", user_id)
}
