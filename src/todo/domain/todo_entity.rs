use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct CreateEntryData {
    pub title: String,
    pub date: i64
}

#[derive(Deserialize, Clone)]
pub struct UpdateEntryData {
    pub title: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodolistEntry {
    pub id: i32,
    pub date: i64,
    pub title: String
}