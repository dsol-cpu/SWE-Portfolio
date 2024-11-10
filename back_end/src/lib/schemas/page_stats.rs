pub struct PageStats {
    pub times_visited: i64,
    pub unique_visitors: i64,
    pub country_visitors: Hashmap<String, i64>,
}
