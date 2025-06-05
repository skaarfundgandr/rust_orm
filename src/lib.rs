pub mod data;
pub mod services;
pub mod models;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_conn_async() {
        let _conn = data::database::connect_to_db_async();
    }
    #[test]
    fn test_pooled_conn() {
        let mut _conn = data::database::connect_from_pool();
    }
}
