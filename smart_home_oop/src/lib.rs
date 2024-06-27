pub trait Object {
    fn search(&self, keyword: &str);
}

// File

pub struct File {
    name: String,
}

impl File {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Object for File {
    fn search(&self, keyword: &str) {
        println!("Searching for keyword {} in file {}", keyword, self.name);
    }
}

// Folder

pub struct Folder {
    name: String,
    objects: Vec<Box<dyn Object>>,
}

impl Folder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            objects: vec![],
        }
    }

    pub fn add(&mut self, object: impl Object + 'static) {
        self.objects.push(Box::new(object));
    }
}

impl Object for Folder {
    fn search(&self, keyword: &str) {
        println!(
            "Searching recursively for keyword {} in folder {}",
            keyword, self.name
        );

        for object in self.objects.iter() {
            object.search(keyword);
        }
    }
}

// Disk

pub struct Disk<T> {
    name: String,
    size: T,
    folders: Vec<Folder>,
}

impl<T> Disk<T> {
    pub fn new(name: &str, size: T) -> Self {
        Self {
            name: name.to_string(),
            size,
            folders: vec![],
        }
    }

    pub fn add(&mut self, folder: Folder) {
        self.folders.push(folder);
    }
}

impl<T: std::fmt::Display> Object for Disk<T> {
    fn search(&self, keyword: &str) {
        println!(
            "Searching recursively for keyword {} in disk {} of size {}",
            keyword, self.name, self.size
        );

        for folder in self.folders.iter() {
            folder.search(keyword);
        }
    }
}
