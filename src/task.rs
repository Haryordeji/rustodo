use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::date_serializer;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub description: String,
    pub completed: bool,
    #[serde(with = "date_serializer")]
    pub date: NaiveDate,
}
