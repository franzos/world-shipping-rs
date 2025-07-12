# Calculate Shipping Rates Worldwide

This is meant to be used to calculate standard shipping rates wordwide, based on origin country and destination country, size and weight, and required level of service.

There's a couple of things to keep in mind:
- You can often get better rates as business customer (frequency, volume, etc.)
- If you have an account with a provider, it might be more accurate, to use their API
- Special cases, like islands, remote areas, etc. are not considered at the moment (you'll get the default rate)

## Warning

I cannot guarantee that this is 100% accurate, nor up to date.

Best practice:

- Check the rates (JSON)
- Run tests for your specific use case

If something is off, I'd appreciate a PR.

## Usage

```rs
let db = ShippingDatabase::from_file("shipping_rates.json").unwrap();
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
```

### Supported inputs:

The available providers and service levels, depend on the country of origin.

- Countries:
  - "DE" (Germany)
    - `Provider::DHL`
      - `ServiceLevel::Standard`
      - `ServiceLevel::Express`
    - `Provider::DPD`
      - `ServiceLevel::Standard`
  - "FR" (France)
    - `Provider::LaPoste`
      - `ServiceLevel::Standard`

If you are shipping from Germany, or France, this package will have some use for you; Otherwise, feel free to add more countries, providers and service levels to the JSON file and submit a PR.

### TypeScript Types

Generate TypeScript types using [typeshare](https://1password.github.io/typeshare/):

```bash
cargo install typeshare-cli
typeshare . --lang=typescript --output-file=types.ts
```


## Test

```bash
RUST_LOG=debug cargo test -- --nocapture
RUST_LOG=debug cargo test -- --test-threads=1 --nocapture
```