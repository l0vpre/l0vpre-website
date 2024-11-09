use serde::Deserialize;


#[derive(Deserialize)]
pub struct NewCommissionForm {
    pub title: String, 
    pub contact: String,
    pub description: String,

}

