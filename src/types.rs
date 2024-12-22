use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Provider {
    #[serde(rename = "ups")]
    UPS,
    #[serde(rename = "fedex")]
    FedEx,
    #[serde(rename = "dhl")]
    DHL,
    #[serde(rename = "usps")]
    USPS,
    #[serde(rename = "la_poste")]
    LaPoste,
    #[serde(rename = "dpd")]
    DPD,
}

// String = Carrier service level
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProviderInfo {
    pub name: String,
    pub id: Provider,
    pub services: Vec<ServiceInfo>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum ServiceLevel {
    Standard,
    Express,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceInfo {
    pub service: ServiceLevel,
    // #[serde(default)]
    // pub country: Option<String>,
    #[serde(default)]
    pub zone: Option<String>,
    pub shipping_time_days: HashMap<String, ShippingTime>,
    pub rates: Vec<RateInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShippingTime {
    pub min: u8,
    pub max: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RateInfo {
    pub name: String,
    pub max_weight: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_dimensions: Option<Dimensions>,
    #[serde(default)]
    pub insurance_included: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insurance_amount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insurance_optional: Option<Vec<InsuranceOption>>,
    #[serde(default)]
    pub tracking_included: bool,
    pub rate: Vec<Rate>,
    #[serde(default)]
    pub vat_exemption: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_exemption_text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dimensions {
    pub length: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub length_width_height_max: Option<u32>,
    pub longest_side_max: Option<u32>,
    pub shortest_longest_side_max: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InsuranceOption {
    pub online_price: f64,
    pub insurance_amount: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rate {
    pub countries: Vec<String>,
    pub online_price: f64,
    #[serde(default)]
    pub vat_exemption: bool,
}

#[derive(Debug, Clone)]
pub struct Region {
    pub country: String,
    pub region: Option<String>,
}