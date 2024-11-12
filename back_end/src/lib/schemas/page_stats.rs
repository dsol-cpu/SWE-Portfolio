pub struct PageStats {
    pub times_visited: i64,
    pub unique_visitors: i64,
    pub country_visitors: std::collections::HashMap<String, i64>,
}
