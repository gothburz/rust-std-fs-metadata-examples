use std::fs;
use std::os::windows::fs::MetadataExt;

fn main() {
    const FILE: &str = "./examples/hello.txt";
    let metadata = fs::metadata(FILE).unwrap();
    println!("{:?}", metadata);
    // Filetype
    let file_type = metadata.file_type();
    println!("{:?}", file_type);
    // is_dir()
    let is_dir_check = metadata.is_dir();
    println!("{}", is_dir_check);
    // is_file()
    let is_file_check = metadata.is_file();
    println!("{}", is_file_check);
    // is_symlink()
    let is_symlink_check = metadata.is_symlink();
    println!("{}", is_symlink_check);
    // len()
    let byte_len = metadata.len();
    println!("{}", byte_len);
    // permissions()
    let file_permissions = metadata.permissions();
    println!("{:?}", file_permissions);
    // modified()
    let modified_time = metadata.modified().unwrap();
    println!("{:?}", modified_time);
    // accessed()
    let accessed_time = metadata.accessed().unwrap();
    println!("{:?}", accessed_time);
    // created()
    let created_time = metadata.created().unwrap();
    println!("{:?}", created_time);

    // TRAIT IMPLEMENTATIONS
    // clone()
    let mut metadata_clone = metadata.clone();
    println!("{:?}", metadata_clone);

    // clone_from()
    metadata_clone.clone_from(&metadata);
    println!("{:?}", metadata_clone);

    // METADATAEXT - WINDOWS

    let msft_file_attributes_value = metadata.file_attributes();
    println!("{:?}", msft_file_attributes_value);
    let msft_file_creation_time_value = metadata.creation_time();
    println!("{:?}", msft_file_creation_time_value);
    let msft_file_last_access_time_value = metadata.last_access_time();
    println!("{:?}", msft_file_last_access_time_value);
    let msft_file_last_write_time_value = metadata.last_write_time();
    println!("{:?}", msft_file_last_write_time_value);
    let msft_file_size_value = metadata.file_size();
    println!("{:?}", msft_file_size_value);
    let msft_volume_serial_number_value = metadata.volume_serial_number();
    println!("{:?}", msft_volume_serial_number_value);
    let msft_number_of_links_value = metadata.number_of_links();
    println!("{:?}", msft_number_of_links_value);
    let msft_file_index_value = metadata.file_index();
    println!("{:?}", msft_file_index_value);




}
