use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::types::{Dimensions, Provider, ProviderInfo, RateInfo, Region, ServiceInfo, ServiceLevel};
use super::types::Rate;

// Item to be shipped
#[typeshare]
#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct ShippingItem {
    pub identifier: String,
    pub weight: Option<u32>,
    pub length: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl ShippingItem {
    // Sum of length, width, and height
    fn length_width_height(&self) -> Option<u32> {
        let mut dimension = 0;
        if let Some(length) = self.length {
            dimension += length;
        }
        if let Some(width) = self.width {
            dimension += width;
        }
        if let Some(height) = self.height {
            dimension += height;
        }
        if dimension == 0 {
            return None;
        }
        Some(dimension)
    }
    // Shortest side of the item
    fn shortest_side(&self) -> Option<u32> {
        // sort available dimensions (length, width, height)
        // return the shortest side
        let mut dimensions = Vec::new();
        if let Some(length) = self.length {
            dimensions.push(length);
        }
        if let Some(width) = self.width {
            dimensions.push(width);
        }
        if let Some(height) = self.height {
            dimensions.push(height);
        }
        if dimensions.is_empty() {
            return None;
        }
        dimensions.sort();
        Some(dimensions[0])
    }
    // Longest side of the item
    fn longest_side(&self) -> Option<u32> {
        // sort available dimensions (length, width, height)
        // return the longest side
        let mut dimensions = Vec::new();
        if let Some(length) = self.length {
            dimensions.push(length);
        }
        if let Some(width) = self.width {
            dimensions.push(width);
        }
        if let Some(height) = self.height {
            dimensions.push(height);
        }
        if dimensions.is_empty() {
            return None;
        }
        dimensions.sort();
        Some(dimensions[dimensions.len() - 1])
    }
    // Length of the shortest + longest side
    fn shortest_longest_side(&self) -> Option<u32> {
        if let Some(shortest_side) = self.shortest_side() {
            if let Some(longest_side) = self.longest_side() {
                return Some(shortest_side + longest_side);
            }
            return None;
        }
        None
    }
    fn is_smaller_or_equal_length_with_height_max(&self, length_width_height_max: Option<u32>) -> bool {
        if let Some(length_width_height_max) = length_width_height_max {
            // let longest_side = self.longest_side();
            if let Some(longest_side) = self.length_width_height() {
                if longest_side <= length_width_height_max {
                    return true;
                }
            }
        } else {
            return true;
        }
        false
    }
    fn is_smaller_or_equal_longest_side_max(&self, longest_side_max: Option<u32>) -> bool {
        if let Some(longest_side_max) = longest_side_max {
            if let Some(longest_side) = self.longest_side() {
                if longest_side <= longest_side_max {
                    return true;
                }
            }
        } else {
            return true;
        }
        false
    }
    fn is_smaller_or_equal_shortest_longest_side_max(&self, shortest_longest_side_max: Option<u32>) -> bool {
        if let Some(shortest_longest_side_max) = shortest_longest_side_max {
            if let Some(longest_side) = self.shortest_longest_side() {
                if longest_side <= shortest_longest_side_max {
                    return true;
                }
            }
        } else {
            return true;
        }
        false
    }
    fn is_smaller_or_equal_max_dimensions(&self, max_dimensions: Option<Dimensions>) -> bool {
        if let Some(max_dimensions) = max_dimensions {
            let mut item_dimensions = Vec::new();
            if let Some(length) = self.length {
                item_dimensions.push(length);
            }
            if let Some(width) = self.width {
                item_dimensions.push(width);
            }
            if let Some(height) = self.height {
                item_dimensions.push(height);
            }
            item_dimensions.sort_by(|a, b| b.cmp(a));

            let mut max_dims = Vec::new();
            if let Some(length) = max_dimensions.length {
                max_dims.push(length);
            }
            if let Some(width) = max_dimensions.width {
                max_dims.push(width);
            }
            if let Some(height) = max_dimensions.height {
                max_dims.push(height);
            }
            max_dims.sort_by(|a, b| b.cmp(a));

            for i in 0..item_dimensions.len().min(max_dims.len()) {
                if item_dimensions[i] > max_dims[i] {
                    return false;
                }
            }
        }
        true
    }
    fn is_larger_or_equal_max_weight(&self, max_weight: Option<u32>) -> bool {
        if let Some(max_weight) = max_weight {
            if let Some(weight) = self.weight {
                if weight > max_weight {
                    return false;
                }
            }
        }
        true
    }
    pub fn is_rate_match(&self, rate_info: &RateInfo) -> bool {
        if !self.is_larger_or_equal_max_weight(Some(rate_info.max_weight)) {
            return false;
        }
        if !self.is_smaller_or_equal_max_dimensions(rate_info.max_dimensions.clone()) {
            return false;
        }
        if rate_info.max_dimensions.is_some() {
            if !self.is_smaller_or_equal_length_with_height_max(rate_info.clone().max_dimensions.unwrap().length_width_height_max) {
                return false;
            }
            if !self.is_smaller_or_equal_longest_side_max(rate_info.clone().max_dimensions.unwrap().longest_side_max) {
                return false;
            }
            if !self.is_smaller_or_equal_shortest_longest_side_max(rate_info.clone().max_dimensions.unwrap().shortest_longest_side_max) {
                return false;
            }
        }
        true
    }
}

// Query to get shipping rates
#[typeshare]
#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct ShippingRateQuery {
    pub source_region: Region,
    pub destination_region: Region,
    pub items: Vec<ShippingItem>,
    pub provider: Option<Provider>,
    pub service_level: Option<ServiceLevel>,
}

#[typeshare]
#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct ApplicableService {
    pub provider: ProviderInfo,
    pub service: ServiceInfo,
    pub rate_info: RateInfo,
    pub rate: Rate,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingRateItemResult {
    pub item_identifier: String,
    pub applicable_services: Vec<ApplicableService>,
}

#[typeshare]
#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct ShippingRateQueryResult {
    pub items: Vec<ShippingRateItemResult>,
    pub total_cost: f64,
}

// String = Country code
#[typeshare]
#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct ShippingDatabase {
    countries: HashMap<String, Vec<ProviderInfo>>,
}

impl ShippingDatabase {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let countries_data = std::fs::read_to_string(path)?;

        let countries: HashMap<String, Vec<ProviderInfo>> = serde_json::from_str(&countries_data)?;

        Ok(Self { countries })
    }

    fn get_country_services(&self, country_code: &str) -> Option<&Vec<ProviderInfo>> {
        self.countries.get(country_code)
    }

    fn filter_by_provider(&self, providers: &Vec<ProviderInfo>, provider: Option<Provider>) -> Option<Vec<ProviderInfo>> {
        if let Some(provider) = provider {
            let mut filtered_providers = Vec::new();
            for provider_info in providers {
                if provider_info.id == provider {
                    filtered_providers.push(provider_info.clone());
                }
            }
            if filtered_providers.is_empty() {
                return None;
            }
            return Some(filtered_providers);
        }
        Some(providers.clone())
    }
    
    fn filter_provider_by_service_level(&self, providers: &Vec<ProviderInfo>, service_level: Option<ServiceLevel>) -> Option<Vec<ProviderInfo>> {
        if let Some(service_level) = service_level {
            let mut filtered_providers = Vec::new();
            for provider in providers {
                let mut updated_provider = provider.clone();
                let mut updated_services = Vec::new();
                for service in &provider.services {
                    if service.service == service_level {
                        updated_services.push(service.clone());
                    }
                }
                if !updated_services.is_empty() {
                    updated_provider.services = updated_services;
                    filtered_providers.push(updated_provider);
                }
            }
            if !filtered_providers.is_empty() {
                Some(filtered_providers)
            } else {
                None
            }
        } else {
            Some(providers.clone())
        }  
    }

    fn match_rate_country(&self, rate: &Vec<Rate>, destination: &Region) -> Option<Rate> {
        for rate in rate {
            for country in &rate.countries {
                if country == &destination.country {  // Add & to compare references
                    return Some(rate.clone());
                }
            }
        }
        None
    }

    fn match_services_with_shipping_item(&self, providers: &Vec<ProviderInfo>, item: &ShippingItem, destination: &Region) -> Option<Vec<ApplicableService>> {
        let mut applicable_services = Vec::new();
        for provider in providers {
            for service in &provider.services {
                for rate_info in &service.rates {
                    if item.is_rate_match(rate_info) {
                        let rate_country_match = self.match_rate_country(&rate_info.rate, destination);
                        if rate_country_match.is_none() {
                            continue;
                        }
                        applicable_services.push(ApplicableService {
                            provider: provider.clone(),
                            service: service.clone(),
                            rate_info: rate_info.clone(),
                            rate: rate_country_match.unwrap(),
                        });
                    }
                }
            }
        }
        
        if applicable_services.is_empty() {
            return None;
        }

        Some(applicable_services)
    }

    pub fn get_rates(&self, query: &ShippingRateQuery) -> Result<Vec<ShippingRateItemResult>, Box<dyn std::error::Error>> {
        // 1. Get the country services for the source country
        let providers = self.get_country_services(&query.source_region.country).ok_or("Country not found")?;
        let filtered_by_provider = self.filter_by_provider(providers, query.provider.clone());
        if filtered_by_provider.is_none() {
            return Err("No providers found".into());
        }
        let filtered_by_service_level = self.filter_provider_by_service_level(&filtered_by_provider.unwrap(), query.service_level.clone());
        if filtered_by_service_level.is_none() {
            return Err("No providers found".into());
        }

        let mut results = Vec::new();

        for item in &query.items {
            let applicable_services = self.match_services_with_shipping_item(&filtered_by_service_level.clone().unwrap(), item, &query.destination_region);
            if applicable_services.is_none() {
                continue;
            }
            let item_result = ShippingRateItemResult {
                item_identifier: item.identifier.clone(),
                applicable_services: applicable_services.unwrap(),
            };
            results.push(item_result);
        }

        Ok(results)
    }

    pub fn get_best_rates(&self, query: &ShippingRateQuery) -> Result<Vec<ShippingRateItemResult>, Box<dyn std::error::Error>> {
        // 1. Get the country services for the source country
        let mut results = self.get_rates(query)?;

        let mut best_results = Vec::new();
        for result in &mut results {
            result.applicable_services.sort_by(|a, b| {
                // Use partial_cmp for floating point numbers
                a.rate.online_price
                    .partial_cmp(&b.rate.online_price)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            
            best_results.push(ShippingRateItemResult {
                item_identifier: result.item_identifier.clone(),
                applicable_services: vec![result.applicable_services[0].clone()],
            });
        }

        Ok(best_results)
    }

    pub fn get_total_shipping_cost(&self, query: &ShippingRateQuery) -> Result<f64, Box<dyn std::error::Error>> {
        let mut total_cost = 0.0;
        let best_rates = self.get_best_rates(query)?;
        for result in best_rates {
            for service in &result.applicable_services {
                total_cost += service.rate.online_price;
            }
        }
        Ok(total_cost)
    }

}