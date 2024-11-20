#[derive(Debug)]
pub struct Commission {
    pub id: i64,
    pub title: String,
    pub contact: String,
    pub description: String,
    pub status: String,
}

#[derive(Debug, sqlx::Type)]
pub enum PortfolioCategory {
    Full = 1,
    VTube = 2,
    Dakimakura = 3,
    Custom = 4,
}

#[derive(Debug)]
pub struct PortfolioItem {
    pub id: i64,
    pub category: PortfolioCategory,
    pub title: String,
    pub image_paths: Vec<String>,
}
