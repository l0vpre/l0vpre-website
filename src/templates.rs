use askama::Template;

use crate::db::Commission;

#[derive(Template)]
#[template(path = "commissions.html")]
pub struct  CommissionTemplate{
    pub commissions: Vec<Commission>,
}

#[derive(Template)]
#[template(path = "commissions_new.html")]
pub struct  CommissionNewTemplate{
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct  IndexTemplate{
}


