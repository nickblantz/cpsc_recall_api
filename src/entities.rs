use {
    bson::{oid::ObjectId, UtcDateTime},
    serde::{Deserialize, Serialize},
};

/// Represents a list of recalls
pub type Recalls = Vec<Recall>;

/// Represents a recall as specified by:
/// https://www.cpsc.gov/s3fs-public/RecallRetrievalWebServicesProgrammersGuide20171031.pdf?n_FtdOGyf5GGSKF5qhIMnlCuJTFaKNS7
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Recall {
    /// Unique identifier local to MongoDB
    #[serde(rename = "_id")]
    id: Option<ObjectId>,

    /// Recall identifier
    #[serde(rename = "RecallID")]
    recall_id: Option<i32>,

    /// Recall number
    recall_number: Option<String>,

    /// Date that the recall was issued
    recall_date: UtcDateTime,

    /// Description of the recall
    description: Option<String>,
    #[serde(rename = "URL")]

    /// URL to the cpsc.gov page for the recall
    url: Option<String>,

    /// Title of the recall
    title: Option<String>,

    /// Contact information for consumers
    consumer_contact: Option<String>,

    /// Last date the recall was updated
    last_publish_date: UtcDateTime,

    /// product(s) that the recall targets
    products: Option<Vec<Product>>,

    /// `?? Unsure of purpose ??`
    inconjunctions: Option<Vec<Inconjunction>>,

    /// List of images of recalled product(s)
    images: Option<Vec<Image>>,

    /// List of reported injuries surrounding the recall
    injuries: Option<Vec<Injury>>,

    /// List of firms who manufactur recalled product(s)
    manufacturers: Option<Vec<RecallFirm>>,

    /// List of firms who retail the recalled product(s)
    retailers: Option<Vec<RecallFirm>>,

    /// List of firms who import the recalled product(s)
    importers: Option<Vec<RecallFirm>>,

    /// List of firms who distribute the recalled product(s)
    distributors: Option<Vec<RecallFirm>>,

    /// List of countries where the recalled product(s) are manufactured
    manufacturer_countries: Option<Vec<ManufacturerCountry>>,

    /// List of Universal Product Codes for recalled product(s)
    #[serde(rename = "ProductUPCs")]
    product_upcs: Option<Vec<ProductUPC>>,

    /// List of hazards that the recalled product(s) expose 
    hazards: Option<Vec<Hazard>>,

    /// List of remedies for the recall 
    remedies: Option<Vec<Remedy>>,

    /// List of remedy options for the recall
    remedy_options: Option<Vec<RemedyOption>>,
}

/// Represents a recalled Product
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Product {
    name: Option<String>,
    description: Option<String>,
    model: Option<String>,
    r#type: Option<String>,
    #[serde(rename = "CategoryID")]
    category_id: Option<String>,
    number_of_units: Option<String>,
}

/// Represents an inconjunction
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Inconjunction {
    #[serde(rename = "URL")]
    url: Option<String>,
}

/// Represents an image
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Image {
    #[serde(rename = "URL")]
    url: Option<String>,
    caption: Option<String>,
}

/// Represents an injury
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Injury {
    name: Option<String>,
}

/// Represents a recall firm
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RecallFirm {
    name: Option<String>,
    #[serde(rename = "CompanyID")]
    company_id: Option<String>,
}

/// Represents a manufacturing country
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ManufacturerCountry {
    country: Option<String>,
}

/// Represents a product's universal product code
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ProductUPC {
    upc: Option<String>,
}

/// Represents a hazard
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Hazard {
    name: Option<String>,
    hazard_type: Option<String>,
    #[serde(rename = "HazardTypeID")]
    hazard_type_id: Option<String>,
}

/// Represents a remedy
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Remedy {
    name: Option<String>,
}

/// Represents a remedy option
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RemedyOption {
    option: Option<String>,
}
