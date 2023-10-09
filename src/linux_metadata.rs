use std::fs;
use std::os::linux::fs::MetadataExt;

fn main() {
    const FILE: &str = "./examples/hello.txt";
    let metadata = fs::metadata(FILE).unwrap();
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

    // METADATAEXT - LINUX
    // st_dev()
    let linux_device_id = metadata.st_dev();
    println!("{:?}", linux_device_id);
    // st_ino()
    let linux_ino_value = metadata.st_ino();
    println!("{:?}", linux_ino_value);
    // st_mode()
    let linux_st_mode_value = metadata.st_mode();
    println!("{:?}", linux_st_mode_value);
    // st_nlink()
    let linux_st_nlink_value = metadata.st_nlink();
    println!("{:?}", linux_st_nlink_value);
    // st_uid()
    let linux_st_uid_value = metadata.st_uid();
    println!("{:?}", linux_st_uid_value);
    // st_gid()
    let linux_st_gid_value = metadata.st_gid();
    println!("{:?}", linux_st_gid_value);
    // st_rdev()
    let linux_st_rdev_value = metadata.st_rdev();
    println!("{:?}", linux_st_rdev_value);
    // st_size()
    let linux_st_size_value = metadata.st_size();
    println!("{:?}", linux_st_size_value);
    // st_atime()
    let linux_st_atime_value = metadata.st_atime();
    println!("{:?}", linux_st_atime_value);
    // st_atime_nsec()
    let linux_st_atime_nsec_value = metadata.st_atime_nsec();
    println!("{:?}", linux_st_atime_nsec_value);
    // st_mtime()
    let linux_st_mtime_value = metadata.st_mtime();
    println!("{:?}", linux_st_mtime_value);
    // st_mtime_nsec()
    let linux_st_mtime_nsec_value = metadata.st_mtime_nsec();
    println!("{:?}", linux_st_mtime_nsec_value);
    // st_ctime()
    let linux_st_ctime_value = metadata.st_ctime();
    println!("{:?}", linux_st_ctime_value);
    // st_ctime_nsec()
    let linux_st_ctime_nsec_value = metadata.st_ctime_nsec();
    println!("{:?}", linux_st_ctime_nsec_value);
    // st_blksize()
    let linux_st_blksize_value = metadata.st_blksize();
    println!("{:?}", linux_st_blksize_value);
    // st_blocks()
    let linux_st_blocks_value = metadata.st_blocks();
    println!("{:?}", linux_st_blocks_value);
}
