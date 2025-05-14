# OpenMenuStandard - Example Usage

This file demonstrates how to use the OpenMenuStandard Rust implementation for common scenarios. These examples cover creating, parsing, and manipulating OMS documents.

## Basic Example

```rust
use open_menu_standard::{
    OmsDocument, Metadata, Vendor, Item, 
    create_minimal_document, parse_oms_document
};
use chrono::Utc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Method 1: Create a minimal document using the helper function
    let doc1 = create_minimal_document(
        "burger-place",
        "Burger Place",
        "restaurant",
        "cheeseburger",
        "Classic Cheeseburger",
        "burger"
    )?;
    
    // Method 2: Create a document manually with more control
    let metadata = Metadata {
        created: Utc::now(),
        source: "example-app".to_string(),
        locale: "en-US".to_string(),
    };
    
    let vendor = Vendor {
        id: "pizza-palace".to_string(),
        name: "Pizza Palace".to_string(),
        r#type: "restaurant".to_string(),
        location_id: Some("downtown".to_string()),
        location_name: Some("Downtown Location".to_string()),
        address: None,
        contact: None,
        hours: None,
        cuisine: None,
        services: None,
    };
    
    let item = Item {
        id: "pepperoni".to_string(),
        name: "Pepperoni Pizza".to_string(),
        category: "pizza".to_string(),
        vendor_id: None,
        description: Some("Classic pepperoni pizza with our signature sauce".to_string()),
        subcategory: None,
        image_url: None,
        base_price: Some(12.99),
        currency: Some("USD".to_string()),
        nutrition: None,
        customizations: None,
        selected_customizations: None,
        quantity: None,
        item_note: None,
        calculated: None,
        components: None,
        availability: None,
        popularity: None,
    };
    
    let doc2 = OmsDocument::new(metadata, vendor, vec![item]);
    
    // Serialize to JSON
    let json1 = doc1.to_json()?;
    let json2 = doc2.to_json()?;
    
    println!("Document 1 JSON:\n{}\n", json1);
    println!("Document 2 JSON:\n{}\n", json2);
    
    // Parse JSON back to a document
    let parsed_doc = parse_oms_document(&json1)?;
    println!("Parsed vendor name: {}", parsed_doc.vendor.name);
    
    // Generate OMS URLs
    println!("Document 1 URL: {}", doc1.create_url().unwrap());
    println!("Document 2 URL: {}", doc2.create_url().unwrap());
    
    Ok(())
}
```

## Creating a Coffee Shop Menu

```rust
use open_menu_standard::{
    OmsDocument, Metadata, Vendor, Item, Customization,
    CustomizationType, CustomizationDefault, CustomizationOption,
    Nutrition, MeasurementValue, NutrientWithDetails
};
use chrono::Utc;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create metadata
    let metadata = Metadata {
        created: Utc::now(),
        source: "coffee-shop-app".to_string(),
        locale: "en-US".to_string(),
    };
    
    // Create vendor
    let vendor = Vendor {
        id: "java-junction".to_string(),
        name: "Java Junction".to_string(),
        r#type: "coffee-shop".to_string(),
        location_id: Some("downtown-3".to_string()),
        location_name: None,
        address: None,
        contact: None,
        hours: None,
        cuisine: None,
        services: None,
    };
    
    // Create size customization
    let size_customization = Customization {
        id: "size".to_string(),
        name: "Size".to_string(),
        r#type: CustomizationType::SingleSelect,
        required: true,
        default: CustomizationDefault::String("medium".to_string()),
        min_selections: None,
        max_selections: None,
        min: None,
        max: None,
        step: None,
        unit_price_adjustment: None,
        unit_nutrition_adjustments: None,
        options: Some(vec![
            CustomizationOption {
                id: "small".to_string(),
                name: "Small (12oz)".to_string(),
                price_adjustment: Some(-0.50),
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: None,
            },
            CustomizationOption {
                id: "medium".to_string(),
                name: "Medium (16oz)".to_string(),
                price_adjustment: Some(0.0),
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: None,
            },
            CustomizationOption {
                id: "large".to_string(),
                name: "Large (20oz)".to_string(),
                price_adjustment: Some(0.50),
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: None,
            },
        ]),
    };
    
    // Create milk customization
    let milk_customization = Customization {
        id: "milk".to_string(),
        name: "Milk Type".to_string(),
        r#type: CustomizationType::SingleSelect,
        required: true,
        default: CustomizationDefault::String("whole".to_string()),
        min_selections: None,
        max_selections: None,
        min: None,
        max: None,
        step: None,
        unit_price_adjustment: None,
        unit_nutrition_adjustments: None,
        options: Some(vec![
            CustomizationOption {
                id: "whole".to_string(),
                name: "Whole Milk".to_string(),
                price_adjustment: Some(0.0),
                nutrition_adjustments: None,
                allergens: Some(vec!["dairy".to_string()]),
                dietary_flags: None,
            },
            CustomizationOption {
                id: "skim".to_string(),
                name: "Skim Milk".to_string(),
                price_adjustment: Some(0.0),
                nutrition_adjustments: None,
                allergens: Some(vec!["dairy".to_string()]),
                dietary_flags: None,
            },
            CustomizationOption {
                id: "almond".to_string(),
                name: "Almond Milk".to_string(),
                price_adjustment: Some(0.75),
                nutrition_adjustments: None,
                allergens: Some(vec!["tree-nuts".to_string()]),
                dietary_flags: Some(vec!["dairy_free".to_string(), "vegan".to_string()]),
            },
            CustomizationOption {
                id: "oat".to_string(),
                name: "Oat Milk".to_string(),
                price_adjustment: Some(0.75),
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: Some(vec!["dairy_free".to_string(), "vegan".to_string()]),
            },
        ]),
    };
    
    // Create shots customization
    let shots_customization = Customization {
        id: "shots".to_string(),
        name: "Espresso Shots".to_string(),
        r#type: CustomizationType::Quantity,
        required: true,
        default: CustomizationDefault::Number(2.0),
        min_selections: None,
        max_selections: None,
        min: Some(1.0),
        max: Some(6.0),
        step: Some(1.0),
        unit_price_adjustment: Some(0.75),
        unit_nutrition_adjustments: None,
        options: None,
    };
    
    // Create nutrition info
    let nutrition = Nutrition {
        serving_size: Some(MeasurementValue {
            value: 16.0,
            unit: "oz".to_string(),
        }),
        calories: Some(180.0),
        protein: Some(MeasurementValue {
            value: 10.0,
            unit: "g".to_string(),
        }),
        fat: Some(NutrientWithDetails {
            value: 9.0,
            unit: "g".to_string(),
            details: None,
        }),
        carbohydrates: Some(NutrientWithDetails {
            value: 15.0,
            unit: "g".to_string(),
            details: None,
        }),
        sodium: None,
        cholesterol: None,
        vitamins: None,
        minerals: None,
        allergens: Some(vec!["dairy".to_string()]),
        dietary_flags: None,
        health_claims: None,
        ingredients: None,
        nutrition_standards: None,
    };
    
    // Create latte item
    let latte = Item {
        id: "latte".to_string(),
        name: "Latte".to_string(),
        category: "coffee".to_string(),
        vendor_id: None,
        description: Some("Espresso with steamed milk".to_string()),
        subcategory: None,
        image_url: None,
        base_price: Some(4.50),
        currency: Some("USD".to_string()),
        nutrition: Some(nutrition),
        customizations: Some(vec![
            size_customization,
            milk_customization,
            shots_customization,
        ]),
        selected_customizations: None,
        quantity: None,
        item_note: None,
        calculated: None,
        components: None,
        availability: None,
        popularity: None,
    };
    
    // Create nutrition info for cappuccino
    let cappuccino_nutrition = Nutrition {
        serving_size: Some(MeasurementValue {
            value: 16.0,
            unit: "oz".to_string(),
        }),
        calories: Some(120.0),
        protein: Some(MeasurementValue {
            value: 8.0,
            unit: "g".to_string(),
        }),
        fat: Some(NutrientWithDetails {
            value: 6.0,
            unit: "g".to_string(),
            details: None,
        }),
        carbohydrates: Some(NutrientWithDetails {
            value: 10.0,
            unit: "g".to_string(),
            details: None,
        }),
        sodium: None,
        cholesterol: None,
        vitamins: None,
        minerals: None,
        allergens: Some(vec!["dairy".to_string()]),
        dietary_flags: None,
        health_claims: None,
        ingredients: None,
        nutrition_standards: None,
    };
    
    // Create cappuccino item
    let cappuccino = Item {
        id: "cappuccino".to_string(),
        name: "Cappuccino".to_string(),
        category: "coffee".to_string(),
        vendor_id: None,
        description: Some("Espresso with equal parts steamed milk and milk foam".to_string()),
        subcategory: None,
        image_url: None,
        base_price: Some(4.25),
        currency: Some("USD".to_string()),
        nutrition: Some(cappuccino_nutrition),
        customizations: Some(vec![
            size_customization.clone(),
            milk_customization.clone(),
            shots_customization.clone(),
        ]),
        selected_customizations: None,
        quantity: None,
        item_note: None,
        calculated: None,
        components: None,
        availability: None,
        popularity: None,
    };
    
    // Create the OMS document with both items
    let document = OmsDocument::new(metadata, vendor, vec![latte, cappuccino]);
    
    // Validate and serialize
    document.validate()?;
    let json = document.to_json()?;
    println!("{}", json);
    
    Ok(())
}
```

## Creating and Processing an Order

```rust
use open_menu_standard::{
    OmsDocument, Metadata, Vendor, Item, Order, OrderStatus, OrderType,
    Payment, PaymentStatus, Customer, SelectedCustomization, CustomizationSelection
};
use chrono::Utc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a minimal document
    let mut doc = open_menu_standard::create_minimal_document(
        "burger-place",
        "Burger Place",
        "restaurant",
        "cheeseburger",
        "Classic Cheeseburger",
        "burger"
    )?;
    
    // Set the price
    if let Some(item) = doc.items.get_mut(0) {
        item.base_price = Some(5.99);
        item.currency = Some("USD".to_string());
        
        // Add selected customizations
        item.selected_customizations = Some(vec![
            SelectedCustomization {
                customization_id: "cheese".to_string(),
                selection: CustomizationSelection::String("cheddar".to_string()),
            },
            SelectedCustomization {
                customization_id: "toppings".to_string(),
                selection: CustomizationSelection::StringArray(vec![
                    "lettuce".to_string(),
                    "tomato".to_string(),
                    "onion".to_string(),
                ]),
            },
        ]);
        
        item.quantity = Some(2);
    }
    
    // Add fries as a second item
    let fries = Item {
        id: "fries".to_string(),
        name: "French Fries".to_string(),
        category: "side".to_string(),
        vendor_id: None,
        description: None,
        subcategory: None,
        image_url: None,
        base_price: Some(2.99),
        currency: Some("USD".to_string()),
        nutrition: None,
        customizations: None,
        selected_customizations: None,
        quantity: Some(1),
        item_note: None,
        calculated: None,
        components: None,
        availability: None,
        popularity: None,
    };
    
    doc.items.push(fries);
    
    // Add order information
    doc.order = Some(Order {
        id: Some("order-123456".to_string()),
        status: Some(OrderStatus::Draft),
        created: Some(Utc::now()),
        pickup_time: Some(Utc::now() + chrono::Duration::minutes(30)),
        delivery_time: None,
        r#type: Some(OrderType::Pickup),
        customer_notes: Some("Please include extra ketchup".to_string()),
        payment: Some(Payment {
            status: Some(PaymentStatus::Unpaid),
            method: Some("credit-card".to_string()),
            subtotal: Some(14.97),
            tax: Some(1.20),
            tip: Some(3.00),
            total: 19.17,
            currency: "USD".to_string(),
        }),
        customer: Some(Customer {
            id: Some("cust-987654".to_string()),
            name: Some("John Doe".to_string()),
            phone: Some("+1-555-123-4567".to_string()),
            email: Some("john.doe@example.com".to_string()),
        }),
        delivery: None,
    });
    
    // Validate and serialize
    doc.validate()?;
    let json = doc.to_json()?;
    println!("{}", json);
    
    // Calculate total price
    if let Some(total) = doc.calculate_total_price() {
        println!("Calculated total: ${:.2}", total);
    }
    
    Ok(())
}
```

## Processing an OMS URL

```rust
use open_menu_standard::{parse_oms_url, create_minimal_document};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a document and generate a URL
    let doc = create_minimal_document(
        "coffee-shop",
        "Coffee Shop",
        "coffee-shop",
        "mocha",
        "Mocha",
        "coffee"
    )?;
    
    let url = doc.create_url().unwrap();
    println!("Generated URL: {}", url);
    
    // Parse the URL
    let params = parse_oms_url(&url).unwrap();
    
    // Use the parameters to load the appropriate item
    let vendor_id = params.get("v").unwrap();
    let item_id = params.get("i").unwrap();
    
    println!("Vendor ID: {}", vendor_id);
    println!("Item ID: {}", item_id);
    
    // In a real application, you would look up the item in a database
    // For this example, we'll just create a new document
    let loaded_doc = create_minimal_document(
        vendor_id,
        "Coffee Shop",
        "coffee-shop",
        item_id,
        "Mocha",
        "coffee"
    )?;
    
    println!("Loaded document for item: {}", loaded_doc.items[0].name);
    
    Ok(())
}
```

## Implementing Tap-to-Order

```rust
use open_menu_standard::{OmsDocument, parse_oms_url, parse_oms_document};
use std::collections::HashMap;

// Simulate reading an NFC tag
fn read_nfc_tag() -> String {
    // In a real application, this would read from an NFC device
    // For this example, we'll just return a hardcoded URL
    "omenu://order?v=pizza-place&l=downtown&i=margherita".to_string()
}

// Simulate a database lookup
fn get_item_from_database(vendor_id: &str, location_id: &str, item_id: &str) -> Result<String, Box<dyn std::error::Error>> {
    // In a real application, this would query a database
    // For this example, we'll create a hardcoded JSON response
    let json = format!(r#"{{
        "oms_version": "1.0",
        "metadata": {{
            "created": "2025-05-14T10:30:00Z",
            "source": "tap-to-order-app",
            "locale": "en-US"
        }},
        "vendor": {{
            "id": "{}",
            "name": "Pizza Place",
            "type": "restaurant",
            "location_id": "{}"
        }},
        "items": [
            {{
                "id": "{}",
                "name": "Margherita Pizza",
                "category": "pizza",
                "base_price": 10.99,
                "currency": "USD",
                "description": "Classic Margherita with tomato sauce, fresh mozzarella, and basil"
            }}
        ]
    }}"#, vendor_id, location_id, item_id);
    
    Ok(json)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the NFC tag
    let tag_content = read_nfc_tag();
    println!("NFC tag read: {}", tag_content);
    
    // Parse the URL
    let params = parse_oms_url(&tag_content).unwrap();
    
    // Extract the parameters
    let action = params.get("action").unwrap();
    let vendor_id = params.get("v").unwrap();
    let location_id = params.get("l").unwrap_or(&"".to_string());
    let item_id = params.get("i").unwrap();
    
    println!("Action: {}", action);
    println!("Vendor: {}", vendor_id);
    println!("Location: {}", location_id);
    println!("Item: {}", item_id);
    
    // Look up the item in the database
    let item_json = get_item_from_database(vendor_id, location_id, item_id)?;
    
    // Parse the item JSON
    let mut document = parse_oms_document(&item_json)?;
    
    // Simulate user customization
    if let Some(item) = document.items.get_mut(0) {
        println!("Item found: {} (${:.2})", item.name, item.base_price.unwrap());
        
        // Add quantity
        item.quantity = Some(2);
        
        // Calculate total
        let total = item.base_price.unwrap() * item.quantity.unwrap() as f64;
        println!("Total for {} x {}: ${:.2}", item.quantity.unwrap(), item.name, total);
    }
    
    // In a real application, you would now allow the user to customize and order
    
    Ok(())
}
```

These examples demonstrate the core functionality of the OpenMenuStandard Rust implementation and provide a starting point for integrating it into various applications.