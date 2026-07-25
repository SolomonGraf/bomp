#[derive(Debug)]
pub enum Construct {
    Word(),
    Pack(i64),
    Structure(Vec<(String, Box<Construct>)>),
}