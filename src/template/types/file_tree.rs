use std::{borrow::Cow, collections::HashMap, path::PathBuf};

use serde::{Deserialize, Deserializer, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum FileTree {
    File(String),
    Directory(HashMap<String, FileTree>),
}

impl FileTree {
    pub fn into_hashmap(self) -> HashMap<String, String> {
        match self {
            Self::File(content) => {
                let mut map = HashMap::new();
                map.insert(String::new(), content);
                map
            }
            Self::Directory(map) => map
                .into_iter()
                .flat_map(|(name, tree)| {
                    let map = tree.into_hashmap();
                    let parent_path = PathBuf::from(&name);

                    map.into_iter().map(move |(name, content)| {
                        let path = if name.is_empty() {
                            Cow::Borrowed(&parent_path)
                        } else {
                            Cow::Owned(parent_path.join(name))
                        };

                        (path.to_string_lossy().to_string(), content)
                    })
                })
                .collect(),
        }
    }
}

pub fn deserialize_into_hashmap<'de, D>(
    deserializer: D,
) -> Result<HashMap<String, String>, D::Error>
where
    D: Deserializer<'de>,
{
    let tree = FileTree::deserialize(deserializer)?;

    let hm = tree.into_hashmap();

    Ok(hm)
}
