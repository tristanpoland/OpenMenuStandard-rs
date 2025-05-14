# OpenMenuStandard Rust Implementation - README

This repository contains a Rust implementation of the OpenMenuStandard (OMS) specification. The library provides a complete data model, validation, serialization/deserialization, and utility functions for working with OpenMenuStandard documents.

## Features

- Complete implementation of the OpenMenuStandard 1.0 specification
- Comprehensive data model with all required and optional fields
- JSON serialization and deserialization using serde
- Document validation using validator
- Utility functions for creating, parsing, and manipulating OMS documents
- Support for OMS URL scheme parsing and generation
- Extensive test coverage

## Getting Started

### Prerequisites

Add the following dependencies to your `Cargo.toml`:

```toml
[dependencies]
open-menu-standard = "0.1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
validator = { version = "0.16", features = ["derive"] }
thiserror = "1.0"
```

### Basic Usage

```rust
use open_menu_standard::{create_minimal_document, parse_oms_document};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a minimal document
    let document = create_minimal_document(
        "subway-usa",
        "Subway",
        "restaurant",
        "italian-bmt",
        "Italian B.M.T.",
        "sandwich"
    )?;
    
    // Serialize to JSON
    let json = document.to_json()?;
    println!("{}", json);
    
    // Parse from JSON
    let parsed_document = parse_oms_document(&json)?;
    
    // Generate a URL for the document
    let url = parsed_document.create_url().unwrap();
    println!("OMS URL: {}", url);
    
    Ok(())
}
```

## Examples

### Creating a Sandwich Menu

This example creates a menu for a sandwich shop with customization options and nutritional information.

```rust
use open_menu_standard::{
    OmsDocument, Metadata, Vendor, Item, Nutrition, Customization,
    CustomizationType, CustomizationDefault, CustomizationOption,
    MeasurementValue, NutrientWithDetails,
};
use chrono::Utc;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create metadata
    let metadata = Metadata {
        created: Utc::now(),
        source: "sandwich-builder-app".to_string(),
        locale: "en-US".to_string(),
    };
    
    // Create vendor
    let vendor = Vendor {
        id: "subway-usa".to_string(),
        name: "Subway".to_string(),
        r#type: "restaurant".to_string(),
        location_id: Some("store-1234".to_string()),
        location_name: Some("Downtown".to_string()),
        address: None,
        contact: None,
        hours: None,
        cuisine: None,
        services: None,
    };
    
    // Create nutrition information
    let mut nutrition = Nutrition {
        serving_size: Some(MeasurementValue {
            value: 240.0,
            unit: "g".to_string(),
        }),
        calories: Some(410.0),
        protein: Some(MeasurementValue {
            value: 22.0,
            unit: "g".to_string(),
        }),
        fat: Some(NutrientWithDetails {
            value: 16.0,
            unit: "g".to_string(),
            details: None,
        }),
        carbohydrates: Some(NutrientWithDetails {
            value: 47.0,
            unit: "g".to_string(),
            details: None,
        }),
        sodium: None,
        cholesterol: None,
        vitamins: None,
        minerals: None,
        allergens: Some(vec!["wheat".to_string(), "dairy".to_string()]),
        dietary_flags: Some(vec!["contains_gluten".to_string()]),
        health_claims: None,
        ingredients: None,
        nutrition_standards: None,
    };
    
    // Create bread customization
    let bread_customization = Customization {
        id: "bread".to_string(),
        name: "Bread Type".to_string(),
        r#type: CustomizationType::SingleSelect,
        required: true,
        default: CustomizationDefault::String("italian-herbs".to_string()),
        min_selections: None,
        max_selections: None,
        min: None,
        max: None,
        step: None,
        unit_price_adjustment: None,
        unit_nutrition_adjustments: None,
        options: Some(vec![
            CustomizationOption {
                id: "italian-herbs".to_string(),
                name: "Italian Herbs & Cheese".to_string(),
                price_adjustment: Some(0.0),
                nutrition_adjustments: None,
                allergens: Some(vec!["wheat".to_string(), "dairy".to_string()]),
                dietary_flags: Some(vec!["contains_gluten".to_string()]),
            },
            CustomizationOption {
                id: "wheat".to_string(),
                name: "Wheat Bread".to_string(),
                price_adjustment: Some(0.0),
                nutrition_adjustments: None,
                allergens: Some(vec!["wheat".to_string()]),
                dietary_flags: Some(vec!["contains_gluten".to_string()]),
            },
            CustomizationOption {
                id: "flatbread".to_string(),
                name: "Flatbread".to_string(),
                price_adjustment: Some(0.0),
                nutrition_adjustments: None,
                allergens: Some(vec!["wheat".to_string()]),
                dietary_flags: Some(vec!["contains_gluten".to_string()]),
            },
        ]),
    };
    
    // Create cheese customization
    let cheese_customization = Customization {
        id: "cheese".to_string(),
        name: "Cheese".to_string(),
        r#type: CustomizationType::SingleSelect,
        required: true,
        default: CustomizationDefault::String("american".to_string()),
        min_selections: None,
        max_selections: None,
        min: None,
        max: None,
        step: None,
        unit_price_adjustment: None,
        unit_nutrition_adjustments: None,
        options: Some(vec![
            CustomizationOption {
                id: "american".to_string(),
                name: "American Cheese".to_string(),
                price_adjustment: Some(0.0),
                nutrition_adjustments: None,
                allergens: Some(vec!["dairy".to_string()]),
                dietary_flags: None,
            },
            CustomizationOption {
                id: "provolone".to_string(),
                name: "Provolone Cheese".to_string(),
                price_adjustment: Some(0.0),
                nutrition_adjustments: None,
                allergens: Some(vec!["dairy".to_string()]),
                dietary_flags: None,
            },
            CustomizationOption {
                id: "none".to_string(),
                name: "No Cheese".to_string(),
                price_adjustment: Some(0.0),
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: None,
            },
        ]),
    };
    
    // Create vegetables customization
    let vegetables_customization = Customization {
        id: "vegetables".to_string(),
        name: "Vegetables".to_string(),
        r#type: CustomizationType::MultiSelect,
        required: false,
        default: CustomizationDefault::StringArray(vec![
            "lettuce".to_string(), 
            "tomato".to_string(), 
            "onion".to_string()
        ]),
        min_selections: Some(0),
        max_selections: Some(10),
        min: None,
        max: None,
        step: None,
        unit_price_adjustment: None,
        unit_nutrition_adjustments: None,
        options: Some(vec![
            CustomizationOption {
                id: "lettuce".to_string(),
                name: "Lettuce".to_string(),
                price_adjustment: Some(0.0),
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: None,
            },
            CustomizationOption {
                id: "tomato".to_string(),
                name: "Tomato".to_string(),
                price_adjustment: Some(0.0),
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: None,
            },
            CustomizationOption {
                id: "onion".to_string(),
                name: "Onion".to_string(),
                price_adjustment: Some(0.0),
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: None,
            },
            CustomizationOption {
                id: "cucumber".to_string(),
                name: "Cucumber".to_string(),
                price_adjustment: Some(0.0),
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: None,
            },
            CustomizationOption {
                id: "peppers".to_string(),
                name: "Bell Peppers".to_string(),
                price_adjustment: Some(0.0),
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: None,
            },
        ]),
    };
    
    // Create sandwich item
    let item = Item {
        id: "italian-bmt".to_string(),
        name: "Italian B.M.T.".to_string(),
        category: "sandwich".to_string(),
        vendor_id: None,
        description: Some("Italian B.M.T.® sandwich with Genoa salami, spicy pepperoni, and Black Forest ham".to_string()),
        subcategory: None,
        image_url: Some("https://example.com/images/italian-bmt.jpg".to_string()),
        base_price: Some(7.99),
        currency: Some("USD".to_string()),
        nutrition: Some(nutrition),
        customizations: Some(vec![
            bread_customization,
            cheese_customization,
            vegetables_customization,
        ]),
        selected_customizations: None,
        quantity: None,
        item_note: None,
        calculated: None,
        components: None,
        availability: None,
        popularity: None,
    };
    
    // Create the OMS document
    let document = OmsDocument::new(metadata, vendor, vec![item]);
    
    // Validate and serialize
    document.validate()?;
    let json = document.to_json()?;
    println!("{}", json);
    
    Ok(())
}
```

### Parsing an OMS URL

```rust
use open_menu_standard::parse_oms_url;

fn main() {
    let url = "omenu://order?v=subway-usa&l=store-1234&i=italian-bmt";
    
    if let Some(params) = parse_oms_url(url) {
        println!("Action: {}", params.get("action").unwrap_or(&"".to_string()));
        println!("Vendor: {}", params.get("v").unwrap_or(&"".to_string()));
        println!("Location: {}", params.get("l").unwrap_or(&"".to_string()));
        println!("Item: {}", params.get("i").unwrap_or(&"".to_string()));
    } else {
        println!("Invalid OMS URL");
    }
}
```

## Setting Up NFC Tags

This library can be used to generate content for NFC tags that implement the tap-to-order functionality in the OpenMenuStandard specification:

```rust
use open_menu_standard::{
    OmsDocument, parse_oms_document, create_minimal_document
};

// Create or load an OMS document
let document = create_minimal_document(
    "burger-joint",
    "Burger Joint",
    "restaurant",
    "cheeseburger",
    "Classic Cheeseburger",
    "burger"
)?;

// Generate an OMS URL for NFC tag
let url = document.create_url().unwrap();
println!("URL for NFC tag: {}", url);

// Alternatively, for more complex data, generate a compact JSON representation
let json = document.to_json()?;
println!("JSON for NFC tag: {}", json);
```

## API Documentation

### Core Types

- `OmsDocument`: The main container for an OpenMenuStandard document
- `Metadata`: Information about the document itself
- `Vendor`: Information about the food service provider
- `Item`: Representation of a food or beverage product
- `Nutrition`: Nutritional information about an item
- `Customization`: Ways in which an item can be modified
- `Order`: Collection of items being ordered

### Main Functions

- `OmsDocument::new()`: Create a new OMS document
- `OmsDocument::validate()`: Validate a document against the specification
- `OmsDocument::to_json()`: Serialize a document to JSON
- `OmsDocument::from_json()`: Deserialize a document from JSON
- `OmsDocument::create_url()`: Generate an OMS URL for the document
- `parse_oms_document()`: Parse a document from JSON
- `create_minimal_document()`: Create a basic document with minimal fields
- `parse_oms_url()`: Parse an OMS URL and extract parameters

## Resources

- [OpenMenuStandard Specification](https://openmenustandard.org)
- [GitHub Repository](https://github.com/openmenustandard/oms-rs)
- [API Documentation](https://docs.rs/open-menu-standard)
- [Crates.io Page](https://crates.io/crates/open-menu-standard)

## License

This library is licensed under the MIT License - see the LICENSE file for details.