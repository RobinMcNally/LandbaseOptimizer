use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
struct Card {
    name: String,
    mana_cost: String,
    cmc: i32,
    data_vector: Vec<String>,
}

pub fn main() {
    unimplemented!()
}

pub fn import_database() {
    unimplemented!()
}
