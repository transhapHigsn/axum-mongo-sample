use bson;
//use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_json::Value;

//#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
//pub enum ListingId {
//    ObjectId,
//    String
//}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListingAndReview {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "listing_url")]
    pub listing_url: String,
    pub name: String,
    pub summary: String,
    pub space: String,
    pub description: String,
    #[serde(rename = "neighborhood_overview")]
    pub neighborhood_overview: String,
    pub notes: String,
    pub transit: String,
    pub access: String,
    pub interaction: String,
    #[serde(rename = "house_rules")]
    pub house_rules: String,
    #[serde(rename = "property_type")]
    pub property_type: String,
    #[serde(rename = "room_type")]
    pub room_type: String,
    #[serde(rename = "bed_type")]
    pub bed_type: String,
    #[serde(rename = "minimum_nights")]
    pub minimum_nights: String,
    #[serde(rename = "maximum_nights")]
    pub maximum_nights: String,
    #[serde(rename = "cancellation_policy")]
    pub cancellation_policy: String,
    #[serde(rename = "last_scraped")]
    pub last_scraped: bson::DateTime,
    #[serde(rename = "calendar_last_scraped")]
    pub calendar_last_scraped: bson::DateTime,
    pub accommodates: i32,
    pub bedrooms: Option<i32>,
    pub beds: i32,
    #[serde(rename = "number_of_reviews")]
    pub number_of_reviews: i32,
    pub bathrooms: bson::Decimal128,
    pub amenities: Vec<String>,
    pub price: bson::Decimal128,
    #[serde(rename = "extra_people")]
    pub extra_people: bson::Decimal128,
    #[serde(rename = "guests_included")]
    pub guests_included: bson::Decimal128,
    pub images: Images,
    pub host: Host,
    pub address: Address,
    pub availability: Availability,
    #[serde(rename = "review_scores")]
    pub review_scores: ReviewScores,
    pub reviews: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Images {
    #[serde(rename = "thumbnail_url")]
    pub thumbnail_url: String,
    #[serde(rename = "medium_url")]
    pub medium_url: String,
    #[serde(rename = "picture_url")]
    pub picture_url: String,
    #[serde(rename = "xl_picture_url")]
    pub xl_picture_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Host {
    #[serde(rename = "host_id")]
    pub host_id: String,
    #[serde(rename = "host_url")]
    pub host_url: String,
    #[serde(rename = "host_name")]
    pub host_name: String,
    #[serde(rename = "host_location")]
    pub host_location: String,
    #[serde(rename = "host_about")]
    pub host_about: String,
    #[serde(rename = "host_thumbnail_url")]
    pub host_thumbnail_url: String,
    #[serde(rename = "host_picture_url")]
    pub host_picture_url: String,
    #[serde(rename = "host_neighbourhood")]
    pub host_neighbourhood: String,
    #[serde(rename = "host_is_superhost")]
    pub host_is_superhost: bool,
    #[serde(rename = "host_has_profile_pic")]
    pub host_has_profile_pic: bool,
    #[serde(rename = "host_identity_verified")]
    pub host_identity_verified: bool,
    #[serde(rename = "host_listings_count")]
    pub host_listings_count: i32,
    #[serde(rename = "host_total_listings_count")]
    pub host_total_listings_count: i32,
    #[serde(rename = "host_verifications")]
    pub host_verifications: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub street: String,
    pub suburb: String,
    #[serde(rename = "government_area")]
    pub government_area: String,
    pub market: String,
    pub country: String,
    #[serde(rename = "country_code")]
    pub country_code: String,
    pub location: Location,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    #[serde(rename = "type")]
    pub type_field: String,
    pub coordinates: Vec<f64>,
    #[serde(rename = "is_location_exact")]
    pub is_location_exact: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Availability {
    #[serde(rename = "availability_30")]
    pub availability_30: i32,
    #[serde(rename = "availability_60")]
    pub availability_60: i32,
    #[serde(rename = "availability_90")]
    pub availability_90: i32,
    #[serde(rename = "availability_365")]
    pub availability_365: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewScores {}
