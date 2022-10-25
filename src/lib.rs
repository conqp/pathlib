static POSIX_PATH_SEP: &str = "/";
static WINDOWS_PATH_SEP: &str = "\\";
static WINDOWS_DRIVE_SEP: &str = ":";

#[derive(Debug)]
pub struct Path {
    drive: Option<String>,
    nodes: Vec<String>,
}

impl Path {
    pub fn new(path: impl Into<String>) -> Self {
        let path = path.into();

        match path.split_once(WINDOWS_DRIVE_SEP) {
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
                .replace(WINDOWS_PATH_SEP, POSIX_PATH_SEP)
                .split(POSIX_PATH_SEP)
                .map(String::from)
                .collect::<Vec<String>>(),
        }
    }

    pub fn parts(&self) -> Vec<String> {
        let mut parts = Vec::new();

        match self.drive() {
            Some(drive) => {
                if self.nodes.get(0).unwrap_or(&"".to_string()).is_empty() {
                    parts.push(format!("{}{}{}", drive, WINDOWS_DRIVE_SEP, WINDOWS_PATH_SEP));
                } else {
                    parts.push(format!("{}{}", drive, WINDOWS_DRIVE_SEP));
                }
            }
            None => ()
        };

        for node in &self.nodes {
            if node.is_empty() {
                if self.drive.is_some() {
                    continue;
                }

                parts.push(POSIX_PATH_SEP.to_string());
            } else {
                parts.push(node.clone());
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
