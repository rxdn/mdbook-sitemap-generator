use serde::ser::{SerializeMap, SerializeStruct};
use serde::{Serialize, Serializer};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct UrlSet {
    pub urls: Vec<Url>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Url {
    pub loc: String,
    pub priority: Option<String>,
}

impl UrlSet {
    pub fn new(urls: Vec<String>) -> Self {
        UrlSet {
            urls: urls
                .into_iter()
                .map(|url| Url {
                    loc: url,
                    priority: Some("1.0".to_string()),
                })
                .collect(),
        }
    }

    pub fn to_xml(&self) -> Result<String, serde_xml_rs::Error> {
        serde_xml_rs::to_string(&self)
    }
}

impl Serialize for UrlSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut obj = serializer.serialize_struct("urlset", self.urls.len())?;
        for url in &self.urls {
            obj.serialize_field("url", url)?;
        }
        obj.end()
    }
}

impl Serialize for Url {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = if self.priority.is_some() { 2 } else { 1 };
        let mut map = serializer.serialize_map(Some(len))?;

        map.serialize_entry("loc", &self.loc)?;
        if let Some(priority) = &self.priority {
            map.serialize_entry("priority", priority)?;
        }

        map.end()
    }
}
