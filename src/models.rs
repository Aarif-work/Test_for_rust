use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Student {
    pub id: i32,
    pub students: String,
    pub math: String,
    pub phy: i32,
    pub chy: i32,
}

#[derive(Debug, Deserialize)]
pub struct NewStudent {
    pub students: String,
    pub math: String,
    pub phy: i32,
    pub chy: i32,
}


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct StudentDetails {
    pub id : i32 ,
    pub age : i32 , 
    pub grade : String, 
    pub address : String,
}


#[derive(Debug, Deserialize)]
pub struct NewStudentDetails {
    pub age : i32 , 
    pub grade : String, 
    pub address : String,
}