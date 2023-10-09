use std::fs;
use std::os::macos::fs::MetadataExt;

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
    // st_dev()
    let macos_device_id = metadata.st_dev();
    println!("{:?}", macos_device_id);
    // st_ino()
    let macos_ino_value = metadata.st_ino();
    println!("{:?}", macos_ino_value);
    // st_mode()
    let macos_st_mode_value = metadata.st_mode();
    println!("{:?}", macos_st_mode_value);
    // st_nlink()
    let macos_st_nlink_value = metadata.st_nlink();
    println!("{:?}", macos_st_nlink_value);
    // st_uid()
    let macos_st_uid_value = metadata.st_uid();
    println!("{:?}", macos_st_uid_value);
    // st_gid()
    let macos_st_gid_value = metadata.st_gid();
    println!("{:?}", macos_st_gid_value);
    // st_rdev()
    let macos_st_rdev_value = metadata.st_rdev();
    println!("{:?}", macos_st_rdev_value);
    // st_size()
    let macos_st_size_value = metadata.st_size();
    println!("{:?}", macos_st_size_value);
    // st_atime()
    let macos_st_atime_value = metadata.st_atime();
    println!("{:?}", macos_st_atime_value);
    // st_atime_nsec()
    let macos_st_atime_nsec_value = metadata.st_atime_nsec();
    println!("{:?}", macos_st_atime_nsec_value);
    // st_mtime()
    let macos_st_mtime_value = metadata.st_mtime();
    println!("{:?}", macos_st_mtime_value);
    // st_mtime_nsec()
    let macos_st_mtime_nsec_value = metadata.st_mtime_nsec();
    println!("{:?}", macos_st_mtime_nsec_value);
    // st_ctime()
    let macos_st_ctime_value = metadata.st_ctime();
    println!("{:?}", macos_st_ctime_value);
    // st_ctime_nsec()
    let macos_st_ctime_nsec_value = metadata.st_ctime_nsec();
    println!("{:?}", macos_st_ctime_nsec_value);
    // st_blksize()
    let macos_st_blksize_value = metadata.st_blksize();
    println!("{:?}", macos_st_blksize_value);
    // st_blocks()
    let macos_st_blocks_value = metadata.st_blocks();
    println!("{:?}", macos_st_blocks_value);

}
