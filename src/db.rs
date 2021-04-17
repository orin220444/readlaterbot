use anyhow::Result;
use rusqlite::Connection;
use rusqlite::NO_PARAMS;
use serde::{de::DeserializeOwned, ser::Serialize};
use serde_rusqlite::*;
pub struct db;
impl db {
    fn connect() -> Result<Connection> {
        let path = std::env::var("DB_URL").expect("Set an DB_URL variable!");
        let conn = Connection::open(&path)?;
        Ok(conn)
    }
    pub fn insert_one<T>(data: &T, table: &str, fields: &str, values: &str) -> Result<i64>
    where
        T: Serialize,
    {
        let conn = Self::connect()?;
        conn.execute_named(
            &format!("INSERT INTO {} ({}) VALUES ({})", table, fields, values),
            &to_params_named(data).unwrap().to_slice(),
        )?;
        Ok(conn.last_insert_rowid())
    }
    pub fn get_all<T>(table: &str, _: &T) -> Result<Vec<T>>
    where
        T: DeserializeOwned,
    {
        let conn = Self::connect()?;
        let mut statement = conn.prepare(&format!("SELECT * FROM {}", table))?;
        let rows = from_rows::<T>(statement.query(NO_PARAMS).unwrap());
        let row = rows.map(|data| data.unwrap()).collect();
        Ok(row)
    }
    pub fn get_specific<T>(table: &str, _: &T, condition: &str) -> Result<Vec<T>>
    where
        T: DeserializeOwned,
    {
        let conn = Self::connect()?;
        let mut statement =
            conn.prepare(&format!("SELECT * FROM {} WHERE {}", table, condition))?;
        let rows = from_rows::<T>(statement.query(NO_PARAMS).unwrap());
        let row = rows.map(|data| data.unwrap()).collect();
        Ok(row)
    }
    pub async fn delete(table: &str, condition: &str) -> Result<()> {
        let conn = Self::connect()?;
        conn.execute(
            &format!("DELETE FROM {} WHERE {}", table, condition),
            NO_PARAMS,
        )?;
        Ok(())
    }
    pub async fn update(table: &str, set: &str, condition: &str) -> Result<()> {
        let conn = Self::connect()?;
        conn.execute(
            &format!("UPDATE {} SET {} WHERE {}", table, set, condition),
            NO_PARAMS,
        )?;
        Ok(())
    }
}
