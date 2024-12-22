# Calculate Shipping Rates Worldwide

This is meant to be used to calculate standard shipping rates wordwide, based on origin country and destination country, size and weight, and required level of service.

## Warning

I cannot guarantee that this is 100% accurate, nor up to date.

Best practice:

- Check the rates (JSON)
- Run tests for your specific use case

If something is off, I'd appreciate a PR.

## Usage

```rs
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
    provider: None,
    service_level: Some(ServiceLevel::Standard),
};

let total_cost = db.get_total_shipping_cost(&query).unwrap();
```

## Test

```bash
RUST_LOG=debug cargo test -- --nocapture
RUST_LOG=debug cargo test -- --test-threads=1 --nocapture
```