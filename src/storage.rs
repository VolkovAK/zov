use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::{Serialize, Deserialize};
use tracing;

use crate::api::GetDirectoryParams;



fn get_dirs_and_nodes(path: &String) -> (Vec<String>, Vec<String>) {
    let mut dirs: Vec<String> = vec![];
    let mut nodes: Vec<String> = vec![];
    let dir_entities = fs::read_dir(path).unwrap();

    for ent in dir_entities {
        let dir_entry = ent.unwrap();
        let file_name = dir_entry.file_name().into_string().unwrap();
        let ent_type = dir_entry.file_type().unwrap();
        if ent_type.is_dir() == true {
            dirs.push(file_name.into());
        }
        else if  ent_type.is_file() == true && file_name.ends_with("meta.json") {
            nodes.push(file_name.into());
        }
    }

    (dirs, nodes)
}


fn parse_storage_from_dir(root_path: &String, cur_dir: &mut ConfigurationDirectory) {
    let cur_storage_path = Path::new(&root_path).join(&cur_dir.full_path).into_os_string().into_string().unwrap();
    tracing::info!("{root_path}, {cur_storage_path}");
    let (dirs, nodes) = get_dirs_and_nodes(&cur_storage_path);
    for node in nodes {
        let full_node_path = Path::new(&cur_storage_path).join(&node).into_os_string().into_string().unwrap();
        tracing::info!("{full_node_path}");
        let config_meta: ConfigurationMetadata = serde_json::from_str(&fs::read_to_string(full_node_path).unwrap()).unwrap();
        // TODO: check for active file, if not found, then use the latest by date of all file for
        // this config
        cur_dir.nodes.insert(node, config_meta);
    }
    for dir in dirs {
        let mut full_dir_path: String = Path::new(&cur_storage_path).join(&dir).into_os_string().into_string().unwrap();
        full_dir_path.push('/');
        let config_dir = ConfigurationDirectory::new()
            .full_path(&full_dir_path)
            .name(&dir);
        cur_dir.dirs.insert(dir.clone(), config_dir);

        let inner_dir = cur_dir.dirs.get_mut(&dir).unwrap();
        parse_storage_from_dir(&root_path, inner_dir);
    }
}



#[derive(Clone, Default)]
pub struct ZovStorage {
    pub root_path: String,
    pub root_dir: ConfigurationDirectory,
}

impl ZovStorage {
    pub fn new(root_path: impl Into<String>) -> Self {
        ZovStorage {
            root_path: root_path.into(),
            root_dir: ConfigurationDirectory::new()
                .full_path("")
                .name(""),
        }
    }

    pub fn set_root_path(&mut self, root_path: impl Into<String>) {
        self.root_path = root_path.into();
    }

    pub fn rebuild_storage_tree(&mut self) {
        parse_storage_from_dir(&self.root_path.clone(), &mut self.root_dir);
    }

    pub fn get_directory(&self, params: &GetDirectoryParams) -> Result<DirectoryRepresentation, String> {
        let path = match &params.path {
            Some(path) => path,
            None => return Err("Directory path not specified".to_string())
        };

        let mut directory: &ConfigurationDirectory = &self.root_dir;
        for subdir in path.split('/') {
            if subdir.len() == 0 {
                continue;
            }
            tracing::info!(subdir);
            let dir = match directory.dirs.get(subdir) {
                Some(dir) => dir,
                None => return Err("No such directory".to_string())
            };
            directory = dir;
        }

        let mut representation = DirectoryRepresentation { nodes: vec![], dirs: vec![] };
        for (dir_name, dir_info) in directory.dirs.iter() {
            representation.dirs.push(TableRowRepresentation { name: dir_name.clone(), change_date: "now".to_string() })

        }

        Ok(DirectoryRepresentation { nodes: vec![], dirs: vec![] })
    }

}

// config directory
#[derive(Clone, Default)]
pub struct ConfigurationDirectory {
    pub full_path: String,
    pub name: String,
    pub nodes: HashMap<String, ConfigurationMetadata>,
    pub dirs: HashMap<String, ConfigurationDirectory>,
}

impl ConfigurationDirectory {
    fn new() -> Self {
        ConfigurationDirectory { 
            full_path: "".to_string(), 
            name: "".to_string(), 
            nodes: HashMap::new(), 
            dirs: HashMap::new(), 
        }
    }

    fn full_path(mut self, full_path: impl Into<String>) -> Self {
        self.full_path = full_path.into();
        self
    }

    fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

}


// config node info
#[derive(Serialize, Deserialize, Clone)]
enum ConfigFileType {
    JSON,
    YAML,
    TEXT,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigurationMetadata {
    full_path: String,
    name: String,
    active_config_name: String,
    file_type: ConfigFileType,
}

// actual config node, one history item
#[derive(Serialize, Deserialize)]
pub struct Configuration {
    author: String,
    change_date: String,
    payload: String,
}


// Data on Site representation
#[derive(Clone, Serialize, Deserialize)]
pub struct TableRowRepresentation {
    name: String,
    change_date: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DirectoryRepresentation {
    nodes: Vec<TableRowRepresentation>,
    dirs: Vec<TableRowRepresentation>,
}

