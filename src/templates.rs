use askama::Template;

use crate::model::Commission;

#[derive(Template)]
#[template(path = "commissions.html")]
pub struct CommissionTemplate {
    pub commissions: Vec<Commission>,
}

#[derive(Template)]
#[template(path = "commissions_new.html")]
pub struct CommissionNewTemplate {
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
}

#[derive(Template)]
#[template(path = "portfolio.html")]
pub struct PortfolioTemplate {
}

#[derive(Template)]
#[template(path = "oc.html")]
pub struct OCTemplate {
}

#[derive(Template)]
#[template(path = "not_found.html")]
pub struct NotFoundTemplate {
}

