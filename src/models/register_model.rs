use serde::Deserialize;


#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegisterModel
{
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String
}
