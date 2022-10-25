pub struct Path {
    drive: Option<String>,
    nodes: Vec<String>,
}

impl Path {
    pub fn new(path: impl Into<String>) -> Self {
        let path = path.into();

        match path.split_once(':') {
            Some((drive, path)) => {
                Self::from_drive_and_path(Some(String::from(drive)), String::from(path))
            }
            None => Self::from_drive_and_path(None, path),
        }
    }

    fn from_drive_and_path(drive: Option<String>, path: String) -> Self {
        Self {
            drive,
            nodes: path
                .split('/')
                .map(String::from)
                .collect::<Vec<String>>(),
        }
    }

    pub fn parts(&self) -> Vec<&str> {
        let mut parts = Vec::new();

        self.drive.as_ref().and_then(|drive| {
            parts.push(drive.as_str());
            None::<String>
        });

        for node in &self.nodes {
            if node.is_empty() {
                parts.push("/")
            } else {
                parts.push(node.as_str())
            }
        }

        parts
    }

    pub fn drive(&self) -> &Option<String> {
        &self.drive
    }

    pub fn root(&self) -> String {
        match self.nodes.get(0) {
            Some(string) => {
                if string.is_empty() {
                    "/".to_string()
                } else {
                    "".to_string()
                }
            }
            None => "".to_string(),
        }
    }

    pub fn anchor(&self) -> String {
        match self.drive() {
            Some(drive) => String::from(drive) + &self.root(),
            None => self.root(),
        }
    }

    pub fn parents(&self) -> Vec<Path> {
        let mut parents = Vec::new();

        for offset in 1..(self.nodes.len() - 1) {
            parents.push(Self {
                drive: self.drive.clone(),
                nodes: Vec::from(&self.nodes[0..(self.nodes.len() - offset)]),
            });
        }

        parents
    }
}
