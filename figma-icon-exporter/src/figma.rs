use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    last_modified: String,
    nodes: HashMap<String, Node>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    document: Document,
}

#[derive(Debug, Serialize, Deserialize)]
struct Document {
    id: String,
    name: String,
    #[serde(rename = "type")]
    doc_type: String,
    #[serde(default)]
    children: Vec<Box<Document>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Images {
    images: HashMap<String, String>,
}
type NodeId = String;
type IconName = String;
type CategoryName = String;
type Url = String;
pub async fn parse_figma_files(
    client: Arc<reqwest::Client>,
    file_key: &str,
    auth_token: &str,
    parent_node_id: &str,
) -> Result<HashMap<CategoryName, Vec<(IconName, Url)>>, Box<dyn std::error::Error>> {
    let res = client
        .get(format!(
            "https://api.figma.com/v1/files/{}/nodes?depth=2&ids={}",
            file_key, parent_node_id
        ))
        .header("X-Figma-Token", auth_token)
        .send()
        .await?;
    let res = res.json::<Response>().await?;

    let mut ids: Vec<String> = vec![];

    let mut nodes: HashSet<(CategoryName, IconName, NodeId)> = HashSet::new();

    for (_, node) in res.nodes.into_iter() {
        node.document.children.iter().for_each(|child| {
            let depth_1 = child.name.replace("&", "").to_case(Case::Snake).clone();
            for child in child.children.iter() {
                let depth_2 = child.name.replace("&", "").to_case(Case::Pascal).clone();
                if child.doc_type == "COMPONENT" {
                    ids.push(child.id.clone());
                    nodes.insert((depth_1.clone(), depth_2, child.id.clone()));
                }
            }
        });
    }
    tracing::debug!("Nodes: {:?}", nodes);
    let res = client
        .get(format!(
            "https://api.figma.com/v1/images/{}?ids={}&format=svg",
            file_key,
            ids.join(",")
        ))
        .header("X-Figma-Token", auth_token)
        .send()
        .await?;
    let image_urls = res.json::<Images>().await?.images;
    let mut result = HashMap::new();
    nodes.into_iter().for_each(|(category, icon, node_id)| {
        if let Some(url) = image_urls.get(&node_id) {
            result
                .entry(category)
                .or_insert_with(Vec::new)
                .push((icon, url.clone()));
        }
    });
    Ok(result)
}

pub async fn fetch_and_parse_svg(
    client: Arc<reqwest::Client>,
    url: &str,
) -> Result<(String, HashMap<String, String>), Box<dyn std::error::Error>> {
    let res = client.get(url).send().await?;
    let svg = res.text().await?;

    let doc = roxmltree::Document::parse(&svg)?;
    let (body, values) = process_node(doc.root_element());
    Ok((body, values))
}
const INDENT: &str = "  ";
const SKIP_ATTR_NAME: [&str; 4] = ["x1", "x2", "y1", "y2"];
fn process_child_node(node: roxmltree::Node, indent_level: usize) -> String {
    let indent = INDENT.repeat(indent_level);
    let mut result = String::new();

    if node.is_element() {
        let tag_name = node.tag_name().name();

        result.push_str(&format!("{}{} {{\n", indent, tag_name));
        if indent_level == 0 {
            if let Some(ns) = node.namespaces().last() {
                result.push_str(&format!("{}  xmlns: \"{}\",\n", indent, ns.uri()));
            }
        }
        for attr in node.attributes() {
            let attr_name = if SKIP_ATTR_NAME.contains(&attr.name()) {
                attr.name().to_string()
            } else {
                attr.name().to_case(convert_case::Case::Snake)
            };
            if indent_level == 0 && attr_name != "view_box" {
                result.push_str(&format!("{}  {},\n", indent, attr_name));
            } else {
                result.push_str(&format!(
                    "{}  {}: \"{}\",\n",
                    indent,
                    attr_name,
                    attr.value()
                ));
            }
        }

        for child in node.children() {
            if child.is_element() {
                result.push_str(&&process_child_node(child, 1));
            }
        }

        result.push_str(&format!("{}}}\n", indent));
    }
    result
}
fn process_node(node: roxmltree::Node) -> (String, HashMap<String, String>) {
    let mut result = String::new();
    let mut values = HashMap::new();
    values.insert("class".to_string(), "".to_string());
    if node.is_element() {
        let tag_name = node.tag_name().name();

        result.push_str(&format!("{} {{\n", tag_name));
        if let Some(ns) = node.namespaces().last() {
            result.push_str(&format!("{}xmlns: \"{}\",\n", INDENT, ns.uri()));
        }
        result.push_str(&format!("{}class,\n", INDENT));
        for attr in node.attributes() {
            let attr_name = attr.name().to_case(convert_case::Case::Snake);
            if attr_name != "view_box" {
                result.push_str(&format!("{}{},\n", INDENT, attr_name));
                values.insert(attr_name, attr.value().to_string());
            } else {
                result.push_str(&format!("{}{}: \"{}\",\n", INDENT, attr_name, attr.value()));
            }
        }

        for child in node.children() {
            if child.is_element() {
                result.push_str(&process_child_node(child, 1));
            }
        }

        result.push_str(&format!("}}\n"));
    }

    (result, values)
}
