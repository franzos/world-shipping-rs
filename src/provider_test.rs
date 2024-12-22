use crate::provider::{ShippingDatabase, ShippingRateQuery, ShippingItem};
use crate::types::{ServiceLevel, Region};

#[test]
fn test_dhl_express_1kg_to_france() {
    let db = ShippingDatabase::from_file("shipping_rates.json").unwrap();
    let query = ShippingRateQuery {
        source_region: Region {
            country: "DE".to_string(),
            region: None,
        },
        destination_region: Region {
            country: "FR".to_string(),
            region: None,
        },
        items: vec![ShippingItem {
            identifier: "test_item".to_string(),
            weight: Some(900),
            length: Some(30),
            width: Some(20),
            height: Some(8),
        }],
        service_level: Some(ServiceLevel::Express),
    };
    
    let results = db.get_rates(&query).unwrap();
    assert!(!results.is_empty());
    let first_result = &results[0];
    assert_eq!(first_result.applicable_services[0].rate.online_price, 55.90);
}

#[test]
fn test_dhl_standard_2kg_to_france() {
    let db = ShippingDatabase::from_file("shipping_rates.json").unwrap();
    let query = ShippingRateQuery {
        source_region: Region {
            country: "DE".to_string(),
            region: None,
        },
        destination_region: Region {
            country: "FR".to_string(),
            region: None,
        },
        items: vec![ShippingItem {
            identifier: "small_package".to_string(),
            weight: Some(1500),
            length: Some(30),
            width: Some(20),
            height: Some(2),
        }],
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
    let db = ShippingDatabase::from_file("shipping_rates.json").unwrap();
    let query = ShippingRateQuery {
        source_region: Region {
            country: "DE".to_string(),
            region: None,
        },
        destination_region: Region {
            country: "FR".to_string(),
            region: None,
        },
        items: vec![ShippingItem {
            identifier: "heavy_package".to_string(),
            weight: Some(35000), // 35kg
            length: Some(60),
            width: Some(30),
            height: Some(15),
        }],
        service_level: Some(ServiceLevel::Standard),
    };
    
    let results = db.get_rates(&query).unwrap();
    assert!(results.is_empty());
}

#[test]
fn test_package_dimensions_check() {
    let db = ShippingDatabase::from_file("shipping_rates.json").unwrap();
    let query = ShippingRateQuery {
        source_region: Region {
            country: "DE".to_string(),
            region: None,
        },
        destination_region: Region {
            country: "FR".to_string(),
            region: None,
        },
        items: vec![ShippingItem {
            identifier: "large_package".to_string(),
            weight: Some(1000),
            length: Some(150), // Exceeds maximum dimensions
            width: Some(80),
            height: Some(70),
        }],
        service_level: Some(ServiceLevel::Standard),
    };
    
    let results = db.get_rates(&query).unwrap();
    assert!(results.is_empty());
}

#[test]
fn test_multiple_items() {
    let db = ShippingDatabase::from_file("shipping_rates.json").unwrap();
    let query = ShippingRateQuery {
        source_region: Region {
            country: "DE".to_string(),
            region: None,
        },
        destination_region: Region {
            country: "FR".to_string(),
            region: None,
        },
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
        service_level: Some(ServiceLevel::Standard),
    };
    
    let results = db.get_rates(&query).unwrap();
    assert_eq!(results.len(), 2);
}

#[test]
fn test_get_best_rates() {
    let db = ShippingDatabase::from_file("shipping_rates.json").unwrap();
    let query = ShippingRateQuery {
        source_region: Region {
            country: "DE".to_string(),
            region: None,
        },
        destination_region: Region {
            country: "FR".to_string(),
            region: None,
        },
        items: vec![
            ShippingItem {
                identifier: "item1".to_string(),
                weight: Some(1500),
                length: Some(30),
                width: Some(20),
                height: Some(2),
            },
        ],
        service_level: Some(ServiceLevel::Standard),
    };
    
    let results = db.get_best_rates(&query).unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].applicable_services.len(), 1);
    assert_eq!(results[0].applicable_services[0].rate.online_price, 6.49);
}

#[test]
fn test_get_total_shipping_cost() {
    let db = ShippingDatabase::from_file("shipping_rates.json").unwrap();
    let query = ShippingRateQuery {
        source_region: Region {
            country: "DE".to_string(),
            region: None,
        },
        destination_region: Region {
            country: "FR".to_string(),
            region: None,
        },
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
        service_level: Some(ServiceLevel::Standard),
    };
    
    let total_cost = db.get_total_shipping_cost(&query).unwrap();
    assert!((total_cost - 22.98).abs() < 0.001);
}

#[test]
fn test_get_best_rates_express() {
    let db = ShippingDatabase::from_file("shipping_rates.json").unwrap();
    let query = ShippingRateQuery {
        source_region: Region {
            country: "DE".to_string(),
            region: None,
        },
        destination_region: Region {
            country: "FR".to_string(),
            region: None,
        },
        items: vec![
            ShippingItem {
                identifier: "express_item".to_string(),
                weight: Some(900),
                length: Some(30),
                width: Some(20),
                height: Some(8),
            },
        ],
        service_level: Some(ServiceLevel::Express),
    };
    
    let results = db.get_best_rates(&query).unwrap();
    assert_eq!(results[0].applicable_services[0].rate.online_price, 55.90);
}
