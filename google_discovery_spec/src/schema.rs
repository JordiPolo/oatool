use std::collections::BTreeMap;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct GoogleSchemas(pub BTreeMap<String, Schema>);

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct GoogleResources(pub BTreeMap<String, Resource>);

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct GoogleParams(pub BTreeMap<String, Property>);

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct GoogleMethods(pub Vec<Method>);

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Spec {
    pub id: String,
    pub name: String,
    pub version: String,
    pub title: String,
    pub description: String,
    #[serde(rename="documentationLink")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub documentation_link: Option<String>,
    pub protocol: String,
    #[serde(rename="basePath")]
    pub base_path: String,
    pub schemas: GoogleSchemas,
    pub resources: GoogleResources,
    #[serde(skip_serializing_if="Option::is_none")]
    pub aliases: Option<Aliases>
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Aliases {
    pub slts: Option<BTreeMap<String, SLT>>,
    pub pagination_params: Option<BTreeMap<String, Property>>
}

// #[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
// pub struct Schema {
//     pub id: String,
//     pub resource: String,
//     #[serde(rename="type")]
//     pub schema_type: String,
//     pub properties: BTreeMap<String, Property>,
// }

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum Schema {
    // TODO: DRY
    ResponseList {
        #[serde(skip_serializing_if="Option::is_none")]
        id: Option<String>,
        #[serde(skip_serializing_if="Option::is_none")]
        resource: Option<String>,
        #[serde(rename="type")]
        schema_type: String,
        items: Reference,
    },
    ResponseSingle {
        id: String,
        resource: String,
        #[serde(rename="type")]
        schema_type: String,
        properties: BTreeMap<String, Property>,
    },
}

#[serde(deny_unknown_fields)]
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Default)]
pub struct Property {
    #[serde(rename="type")]
    pub property_type: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub items: Option<TypeOrReference>,
    // Are default and required valid in this context? Maybe properties can be Properties or Params
    #[serde(rename="default")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub the_default: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub location: Option<String>,
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
    #[serde(skip_serializing_if="Option::is_none")]
    pub parameters: Option<GoogleParams>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub request: Option<MethodRequest>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub response: Option<Response>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub slt: Option<SLT>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct MethodRequest {
    #[serde(rename="$ref")]
    pub location: String,
}

//This is unused because it seems that Parameters can have everything properties can have
// Probably this is incorrect but hey
// #[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Default)]
// pub struct Parameter {
//     #[serde(skip_serializing_if="Option::is_none")]
//     pub description: Option<String>,
//     #[serde(skip_serializing_if="Option::is_none")]
//     pub required: Option<bool>,
//     #[serde(skip_serializing_if="Option::is_none")]
//     pub location: Option<String>,
//    // pub location: String,
//     #[serde(rename="type")]
//     #[serde(skip_serializing_if="Option::is_none")]
//     pub param_type: Option<String>,
//   //  pub referenced_data: Option<String>,
// }

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum Response {
    //TODO: DRY
    Reference {
        #[serde(rename="$ref")]
        location: String,
    },
    ResponseList {
        id: Option<String>,
        resource: Option<String>,
        #[serde(rename="type")]
        response_type: String,
        items: Reference,
    },
    ResponseSingle {
        id: String,
        resource: String,
        #[serde(rename="type")]
        response_type: String,
        properties: BTreeMap<String, Property>,
    },
}



#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct SLT {
    #[serde(rename="99th_percentile")]
    percentile_99th: String,
    std_dev: String,
    requests_per_second: i32,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Reference {
    #[serde(rename="$ref")]
    pub location: String,
}


#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum TypeOrReference {
    //TODO: DRY
    Reference {
        #[serde(rename="$ref")]
        location: String,
    },
    Type {
        #[serde(rename="type")]
        items_type: String,
    }
}

//TODO: Not used anywhere but may be useful to dry the enums?

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct ResponseList {
    pub id: Option<String>,
    pub resource: Option<String>,
    #[serde(rename="type")]
    pub response_type: String,
    pub items: Reference,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct ResponseSingle {
    pub id: String,
    pub resource: String,
    #[serde(rename="type")]
    pub schema_type: String,
    pub properties: BTreeMap<String, Property>,
}
