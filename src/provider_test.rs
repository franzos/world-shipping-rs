use log::debug;
use crate::provider::{ShippingDatabase, ShippingRateQuery, ShippingItem};
use crate::types::{Provider, Region, ServiceLevel};

fn setup() -> ShippingDatabase {
    let _ = env_logger::builder()
        .is_test(true)
        .filter_level(log::LevelFilter::Debug)  // Set to Debug level
        .try_init();
    ShippingDatabase::from_file("shipping_rates.json").unwrap()
}

#[test]
fn test_dhl_express_1kg_to_france() {
    let db = setup();
    let query = ShippingRateQuery {
        source_region: Region::new("DE".to_string(), None).expect("Country code to be valid"),
        destination_region: Region::new("FR".to_string(), None).expect("Country code to be valid"),
        items: vec![ShippingItem {
            identifier: "test_item".to_string(),
            weight: Some(900),
            length: Some(30),
            width: Some(20),
            height: Some(8),
        }],
        provider: None,
        service_level: Some(ServiceLevel::Express),
    };
    
    let results = db.get_rates(&query).unwrap();
    assert!(!results.is_empty());
    let first_result = &results[0];
    assert_eq!(first_result.applicable_services[0].rate.online_price, 55.90);
}

#[test]
fn test_dpd_standard_germany_to_france() {
    let db = setup();
    let query = ShippingRateQuery {
        source_region: Region::new("DE".to_string(), None).expect("Country code to be valid"),
        destination_region: Region::new("FR".to_string(), None).expect("Country code to be valid"),
        items: vec![ShippingItem {
            identifier: "small_package".to_string(),
            weight: Some(1500),
            length: Some(30),
            width: Some(20),
            height: Some(2),
        }],
        provider: Some(Provider::DPD),
        service_level: Some(ServiceLevel::Standard),
    };

    let results = db.get_total_shipping_cost(&query).unwrap();
    assert!((results - 12.90).abs() < 0.001);
}

#[test]
fn test_dhl_standard_2kg_to_france() {
    let db = setup();
    let query = ShippingRateQuery {
        source_region: Region::new("DE".to_string(), None).expect("Country code to be valid"),
        destination_region: Region::new("FR".to_string(), None).expect("Country code to be valid"),
        items: vec![ShippingItem {
            identifier: "small_package".to_string(),
            weight: Some(1500),
            length: Some(30),
            width: Some(20),
            height: Some(2),
        }],
        provider: None,
        service_level: Some(ServiceLevel::Standard),
    };
    
    let results = db.get_rates(&query).unwrap();
    assert!(!results.is_empty());
    let first_service = &results[0].applicable_services[0];
    assert_eq!(first_service.rate_info.max_weight, 2000);
    assert_eq!(first_service.rate.online_price, 6.49);
}

#[test]
fn test_package_too_heavy() {
    let db = setup();
    let query = ShippingRateQuery {
        source_region: Region::new("DE".to_string(), None).expect("Country code to be valid"),
        destination_region: Region::new("FR".to_string(), None).expect("Country code to be valid"),
        items: vec![ShippingItem {
            identifier: "heavy_package".to_string(),
            weight: Some(35000), // 35kg
            length: Some(60),
            width: Some(30),
            height: Some(15),
        }],
        provider: None,
        service_level: Some(ServiceLevel::Standard),
    };
    
    let results = db.get_rates(&query).unwrap();
    assert!(results.is_empty());
}

#[test]
fn test_package_dimensions_check() {
    let db = setup();
    let query = ShippingRateQuery {
        source_region: Region::new("DE".to_string(), None).expect("Country code to be valid"),
        destination_region: Region::new("FR".to_string(), None).expect("Country code to be valid"),
        items: vec![ShippingItem {
            identifier: "large_package".to_string(),
            weight: Some(1000),
            length: Some(150), // Exceeds maximum dimensions
            width: Some(80),
            height: Some(70),
        }],
        provider: None,
        service_level: Some(ServiceLevel::Standard),
    };
    
    let results = db.get_rates(&query).unwrap();
    assert!(results.is_empty());
}

#[test]
fn test_multiple_items() {
    let db = setup();
    let query = ShippingRateQuery {
        source_region: Region::new("DE".to_string(), None).expect("Country code to be valid"),
        destination_region: Region::new("FR".to_string(), None).expect("Country code to be valid"),
        items: vec![
            ShippingItem {
                identifier: "small_item".to_string(),
                weight: Some(500),
                length: Some(20),
                width: Some(15),
                height: Some(2),
            },
            ShippingItem {
                identifier: "medium_item".to_string(),
                weight: Some(4000),
                length: Some(40),
                width: Some(30),
                height: Some(20),
            },
        ],
        provider: None,
        service_level: Some(ServiceLevel::Standard),
    };
    
    let results = db.get_rates(&query).unwrap();
    assert_eq!(results.len(), 2);
}

#[test]
fn test_get_best_rates() {
    let db = setup();
    let query = ShippingRateQuery {
        source_region: Region::new("DE".to_string(), None).expect("Country code to be valid"),
        destination_region: Region::new("FR".to_string(), None).expect("Country code to be valid"),
        items: vec![
            ShippingItem {
                identifier: "item1".to_string(),
                weight: Some(1500),
                length: Some(30),
                width: Some(20),
                height: Some(2),
            },
        ],
        provider: None,
        service_level: Some(ServiceLevel::Standard),
    };
    
    let results = db.get_best_rates(&query).unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].applicable_services.len(), 1);
    assert_eq!(results[0].applicable_services[0].rate.online_price, 6.49);
}

#[test]
fn test_get_total_shipping_cost() {
    let db = setup();
    let query = ShippingRateQuery {
        source_region: Region::new("DE".to_string(), None).expect("Country code to be valid"),
        destination_region: Region::new("FR".to_string(), None).expect("Country code to be valid"),
        items: vec![
            ShippingItem {
                identifier: "item1".to_string(),
                weight: Some(1500),
                length: Some(30),
                width: Some(20),
                height: Some(2),
            },
            ShippingItem {
                identifier: "item2".to_string(),
                weight: Some(4000),
                length: Some(40),
                width: Some(30),
                height: Some(20),
            },
        ],
        provider: None,
        service_level: Some(ServiceLevel::Standard),
    };
    
    let total_cost = db.get_total_shipping_cost(&query).unwrap();
    assert!((total_cost - 22.98).abs() < 0.001);
}

#[test]
fn test_get_best_rates_express() {
    let db = setup();
    let query = ShippingRateQuery {
        source_region: Region::new("DE".to_string(), None).expect("Country code to be valid"),
        destination_region: Region::new("FR".to_string(), None).expect("Country code to be valid"),
        items: vec![
            ShippingItem {
                identifier: "express_item".to_string(),
                weight: Some(900),
                length: Some(30),
                width: Some(20),
                height: Some(8),
            },
        ],
        provider: None,
        service_level: Some(ServiceLevel::Express),
    };
    
    let results = db.get_best_rates(&query).unwrap();
    assert_eq!(results[0].applicable_services[0].rate.online_price, 55.90);
}