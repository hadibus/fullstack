use bigDecimal::BigDecimal;
use crate::db;
use crate::error_handler::CustomError;
use crate::schema::products;
use diesel::prelude::*;
use diesel::data_types::Cents;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "products"]
pub struct Product {
    pub price: Cents,
    pub count: i32,
    pub image: String,
    pub description: String,
    pub name: String,
}
#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "products"]
pub struct Products {
    pub id: i32,
    pub price: Cents,
    pub count: i32,
    pub image: String,
    pub description: String,
    pub name: String,
}
impl Products {
    pub fn find_all() -> Result<Vec, CustomError> {
        let conn = db::connection()?;
        let products = products::table.load::(&conn)?;
        Ok(products)
    }
    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let product = products::table.filter(products::id.eq(id)).first(&conn)?;
        Ok(product)
    }
    pub fn create(product: Product) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let product = Product::from(product);
        let product = diesel::insert_into(products::table)
            .values(product)
            .get_result(&conn)?;
        Ok(product)
    }
    pub fn update(id: i32, product: Product) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let product = diesel::update(products::table)
            .filter(products::id.eq(id))
            .set(product)
            .get_result(&conn)?;
        Ok(product)
    }
    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(products::table.filter(products::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}
impl Product {
    fn from(product: Product) -> Product {
        Product {
            first_name: product.first_name,
            last_name: product.last_name,
            department: product.department,
            salary: product.salary,
            age: product.age,
        }
    }
}
