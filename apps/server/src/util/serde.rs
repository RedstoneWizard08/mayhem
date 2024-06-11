use mayhem_db::sea_orm::DeleteResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SerdeDeleteResult {
    pub rows_affected: u64,
}

impl SerdeDeleteResult {
    pub fn from(res: DeleteResult) -> Self {
        return Self {
            rows_affected: res.rows_affected,
        };
    }
}
