pub mod select;
pub mod insert;
pub mod mutate;

pub fn get_base_data_path() -> String {
    String::from("./.data/")
}
