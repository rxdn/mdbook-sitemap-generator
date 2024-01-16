use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use quick_xml::{DeError, se::to_string};

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename = "urlset")]
pub(crate) struct UrlSet {
    #[serde(rename = "@xlmns")]
    pub xlmns: String,

    pub url: Vec<Url>
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Url {
    pub loc: String,
    pub priority: Option<String>,
}

impl UrlSet {
    pub fn new(urls: Vec<String>) -> Self {
        UrlSet {
            xlmns: "http://www.sitemaps.org/schemas/sitemap/0.9".to_string(),
            url: urls
                .into_iter()
                .map(|url| Url {
                    loc: url.replace(".md",".html"),
                    priority: Some("1.0".to_string()),
                })
                .collect(),
        }
    }

    pub fn to_xml(&self) -> Result<String,DeError> {
        return to_string(&self);
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
