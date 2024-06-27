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
