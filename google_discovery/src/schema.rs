use std::collections::BTreeMap;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Spec {
    pub id: String,
    pub name: String,
    pub version: String,
    pub title: String,
    pub description: String,
    #[serde(rename="documentationLink")]
    pub documentation_link: String,
    pub protocol: String,
    #[serde(rename="basePath")]
    pub base_path: String,
    pub schemas: BTreeMap<String, Schema>,
    pub resources: BTreeMap<String, Resource>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Schema {
    pub id: String,
    pub resource: String,
    #[serde(rename="type")]
    pub schema_type: String,
    pub properties: BTreeMap<String, Property>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Property {
    #[serde(rename="type")]
    pub property_type: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Resource {
    pub methods: BTreeMap<String, Method>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Method {
    pub id: String,
    pub path: String,
    #[serde(rename="httpMethod")]
    pub http_method: String,
    pub description: String,
    pub parameters: BTreeMap<String, Parameter>,
    pub response: ArrayOrRef,
    pub slt: Option<SLT>,
}



#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Parameter {
    pub description: Option<String>,
    pub required: Option<bool>,
    pub location: String,
    #[serde(rename="type")]
    pub param_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum ArrayOrRef {
    SimpleResponse {
        #[serde(rename="$ref")]
        location: String,
    },
    Response {
        #[serde(rename="type")]
        response_type: String,
        items: ItemRef,
    },
    Schema {
        id: String,
        resource: String,
        #[serde(rename="type")]
        schema_type: String,
        properties: BTreeMap<String, Property>,
    },
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct ItemRef {
    #[serde(rename="$ref")]
    pub location: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct SLT {
    #[serde(rename="99th_percentile")]
    percentile_99th: String,
    std_dev: String,
    requests_per_second: i32,
}
