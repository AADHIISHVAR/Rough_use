use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginModel
{
    pub(crate) username_or_email:String,
    pub(crate) password: String
}