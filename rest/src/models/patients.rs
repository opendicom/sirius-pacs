use diesel::prelude::*;
use serde::Serialize;

#[derive(Serialize, Selectable, Queryable,Debug)]
#[diesel(table_name = crate::schema::patients)]
pub struct Patient {
    pub patient_pk: i64,
    pub patient_id: Option<String>,
    pub patient_id_issuer: Option<String>,
    pub patient_fname: Option<String>,
    pub patient_gname: Option<String>,
    pub patient_gender: Option<String>,
}