use oop::{Disk, File, Folder, Object};

fn main() {
    let file1 = File::new("File 1");
    let file2 = File::new("File 2");
    let file3 = File::new("File 3");

    let mut folder1 = Folder::new("Folder 1");
    folder1.add(file1);

    let mut folder2 = Folder::new("Folder 2");
    folder2.add(file2);
    folder2.add(file3);
    folder2.add(folder1);

    let mut disk1 = Disk::new("Disk 1", "100_u8");
    disk1.add(folder2);

    disk1.search("Otus");
}
