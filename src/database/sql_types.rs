use base64::{engine::general_purpose, Engine as _};
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Default)]
pub struct Param {
    pub int32: Option<i32>,
    pub int64: Option<i64>,
    pub float32: Option<f32>,
    pub float64: Option<f64>,
    pub string: Option<String>,
    // pub time: Option<String>, //  rfc3339 encoding
    pub bool: Option<bool>,
    pub bytes: Option<String>, // base64 encoding
}

#[derive(Serialize)]
pub struct DBQuery {
    pub statement: String,
    pub params: Vec<Param>,
}

pub trait SQLType {
    fn get_param(&self) -> Param;
}

impl SQLType for i32 {
    fn get_param(&self) -> Param {
        Param {
            int32: Some(*self),
            ..Default::default()
        }
    }
}

impl SQLType for i64 {
    fn get_param(&self) -> Param {
        Param {
            int64: Some(*self),
            ..Default::default()
        }
    }
}

impl SQLType for f32 {
    fn get_param(&self) -> Param {
        Param {
            float32: Some(*self),
            ..Default::default()
        }
    }
}

impl SQLType for f64 {
    fn get_param(&self) -> Param {
        Param {
            float64: Some(*self),
            ..Default::default()
        }
    }
}

impl SQLType for &str {
    fn get_param(&self) -> Param {
        Param {
            string: Some((*self).to_string()),
            ..Default::default()
        }
    }
}

impl SQLType for bool {
    fn get_param(&self) -> Param {
        Param {
            bool: Some(*self),
            ..Default::default()
        }
    }
}

impl SQLType for Vec<u8> {
    fn get_param(&self) -> Param {
        Param {
            bytes: Some(general_purpose::STANDARD_NO_PAD.encode(self)),
            ..Default::default()
        }
    }
}
