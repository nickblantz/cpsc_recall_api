use {
    bson::UtcDateTime,
    chrono::{DateTime, Utc},
    serde::{Deserialize, Serialize}
};

#[derive(Serialize, Deserialize)]
pub struct Recalls(Vec<Recall>);

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Recall {
    recall_id: Option<usize>,
    recall_number: Option<String>,
    #[serde(with = "sortable_format")]
    recall_date: DateTime<Utc>,
    description: Option<String>,
    url: Option<String>,
    title: Option<String>,
    consumer_contact: Option<String>,
    #[serde(with = "sortable_format")]
    last_publish_date: DateTime<Utc>,
    products: Option<Vec<Product>>,
    inconjunctions: Option<Vec<Inconjunction>>,
    images: Option<Vec<Image>>,
    injuries: Option<Vec<Injury>>,
    manufacturers: Option<Vec<RecallFirm>>,
    retailers: Option<Vec<RecallFirm>>,
    importers: Option<Vec<RecallFirm>>,
    distributors: Option<Vec<RecallFirm>>,
    manufacturer_countries: Option<Vec<ManufacturerCountry>>,
    product_upcs: Option<Vec<ProductUPC>>,
    hazards: Option<Vec<Hazard>>,
    remedies: Option<Vec<Remedy>>,
    remedy_options: Option<Vec<RemedyOption>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Product {
    name: Option<String>,
    description: Option<String>,
    model: Option<String>,
    r#type: Option<String>,
    category_id: Option<String>,
    number_of_units: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Inconjunction {
    url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Image {
    url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Injury {
    name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RecallFirm {
    name: Option<String>,
    company_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ManufacturerCountry {
    country: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ProductUPC {
    upc: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Hazard {
    name: Option<String>,
    hazard_type: Option<String>,
    hazard_type_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Remedy {
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RemedyOption {
    option: Option<String>,
}

impl Recalls {
    pub fn get_recalls(&self) -> &Vec<Recall> {
        &self.0
    }
}

mod sortable_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}
