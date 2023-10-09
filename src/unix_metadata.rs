use std::fs;
use std::os::unix::fs::MetadataExt;

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

    // METADATAEXT - UNIX
    let unix_dev_id_value = metadata.dev();
    println!("{:?}", unix_dev_id_value);
    let unix_ino_value = metadata.ino();
    println!("{:?}", unix_ino_value);
    let unix_mode_value = metadata.mode();
    println!("{:?}", unix_mode_value);
    let unix_nlink_value = metadata.nlink();
    println!("{:?}", unix_nlink_value);
    let unix_uid_value = metadata.uid();
    println!("{:?}", unix_uid_value);
    let unix_gid_value = metadata.gid();
    println!("{:?}", unix_gid_value);
    let unix_size_value = metadata.size();
    println!("{:?}", unix_size_value);
    let unix_atime_value = metadata.atime();
    println!("{:?}", unix_atime_value);
    let unix_atime_nsec_value = metadata.atime_nsec();
    println!("{:?}", unix_atime_nsec_value);
    let unix_mtime_value = metadata.mtime();
    println!("{:?}", unix_mtime_value);
    let unix_mtime_nsec_value = metadata.mtime_nsec();
    println!("{:?}", unix_mtime_nsec_value);
    let unix_ctime_value = metadata.ctime();
    println!("{:?}", unix_ctime_value);
    let unix_ctime_nsec_value = metadata.ctime_nsec();
    println!("{:?}", unix_ctime_nsec_value);
    let unix_blksize_value = metadata.blksize();
    println!("{:?}", unix_blksize_value);
    let unix_num_of_blocks_value = metadata.blocks();
    println!("{:?}", unix_num_of_blocks_value);


}
