use crate::rbatis;


#[crud_table]
#[derive(Clone, Debug)]
pub struct Demo {
  pub id: Option<String>,
  pub name: Option<String>
}
