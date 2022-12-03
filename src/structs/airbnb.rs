use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "_id")]
    pub id: Id,
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
    pub last_scraped: LastScraped,
    #[serde(rename = "calendar_last_scraped")]
    pub calendar_last_scraped: CalendarLastScraped,
    pub accommodates: Accommodates,
    pub bedrooms: Bedrooms,
    pub beds: Beds,
    #[serde(rename = "number_of_reviews")]
    pub number_of_reviews: NumberOfReviews,
    pub bathrooms: Bathrooms,
    pub amenities: Vec<String>,
    pub price: Price,
    #[serde(rename = "extra_people")]
    pub extra_people: ExtraPeople,
    #[serde(rename = "guests_included")]
    pub guests_included: GuestsIncluded,
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
pub struct Id {
    #[serde(rename = "$oid")]
    pub oid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastScraped {
    #[serde(rename = "$date")]
    pub date: Date,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Date {
    #[serde(rename = "$numberLong")]
    pub number_long: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarLastScraped {
    #[serde(rename = "$date")]
    pub date: Date2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Date2 {
    #[serde(rename = "$numberLong")]
    pub number_long: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accommodates {
    #[serde(rename = "$numberInt")]
    pub number_int: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bedrooms {
    #[serde(rename = "$numberInt")]
    pub number_int: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Beds {
    #[serde(rename = "$numberInt")]
    pub number_int: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberOfReviews {
    #[serde(rename = "$numberInt")]
    pub number_int: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bathrooms {
    #[serde(rename = "$numberDecimal")]
    pub number_decimal: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    #[serde(rename = "$numberDecimal")]
    pub number_decimal: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtraPeople {
    #[serde(rename = "$numberDecimal")]
    pub number_decimal: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuestsIncluded {
    #[serde(rename = "$numberDecimal")]
    pub number_decimal: String,
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
    pub host_listings_count: HostListingsCount,
    #[serde(rename = "host_total_listings_count")]
    pub host_total_listings_count: HostTotalListingsCount,
    #[serde(rename = "host_verifications")]
    pub host_verifications: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostListingsCount {
    #[serde(rename = "$numberInt")]
    pub number_int: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostTotalListingsCount {
    #[serde(rename = "$numberInt")]
    pub number_int: String,
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
    pub coordinates: Vec<Coordinate>,
    #[serde(rename = "is_location_exact")]
    pub is_location_exact: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coordinate {
    #[serde(rename = "$numberDouble")]
    pub number_double: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Availability {
    #[serde(rename = "availability_30")]
    pub availability_30: Availability30,
    #[serde(rename = "availability_60")]
    pub availability_60: Availability60,
    #[serde(rename = "availability_90")]
    pub availability_90: Availability90,
    #[serde(rename = "availability_365")]
    pub availability_365: Availability365,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Availability30 {
    #[serde(rename = "$numberInt")]
    pub number_int: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Availability60 {
    #[serde(rename = "$numberInt")]
    pub number_int: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Availability90 {
    #[serde(rename = "$numberInt")]
    pub number_int: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Availability365 {
    #[serde(rename = "$numberInt")]
    pub number_int: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewScores {}
