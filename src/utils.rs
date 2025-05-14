// src/utils.rs
//
// Utility functions for working with OMS documents

use crate::{OMS_VERSION, OmsError, OmsResult};
use crate::types::*;
use crate::document::parse_oms_document;
use crate::url::parse_oms_url;
use chrono::Utc;
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use std::io::{Read, Write};

/// Create a minimal OMS document with basic fields
pub fn create_minimal_document(
    vendor_id: &str,
    vendor_name: &str,
    vendor_type: &str,
    item_id: &str,
    item_name: &str,
    item_category: &str,
) -> OmsResult<OmsDocument> {
    let metadata = Metadata {
        created: Utc::now(),
        source: "open_menu_standard".to_string(),
        locale: "en-US".to_string(),
    };
    
    let vendor = Vendor {
        id: vendor_id.to_string(),
        name: vendor_name.to_string(),
        r#type: vendor_type.to_string(),
        location_id: None,
        location_name: None,
        address: None,
        contact: None,
        hours: None,
        cuisine: None,
        services: None,
    };
    
    let item = Item {
        id: item_id.to_string(),
        name: item_name.to_string(),
        category: item_category.to_string(),
        vendor_id: None,
        description: None,
        subcategory: None,
        image_url: None,
        base_price: None,
        currency: None,
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
    
    let document = OmsDocument::new(metadata, vendor, vec![item]);
    document.validate()?;
    Ok(document)
}

/// Create a template OMS document for a specific vendor type
pub fn create_template(vendor_type: &str) -> OmsResult<OmsDocument> {
    match vendor_type {
        "restaurant" => create_restaurant_template(),
        "cafe" => create_cafe_template(),
        "fast-food" => create_fast_food_template(),
        "coffee-shop" => create_coffee_shop_template(),
        "pizzeria" => create_pizzeria_template(),
        _ => Err(OmsError::InvalidVendorType(vendor_type.to_string())),
    }
}

/// Create a restaurant template
fn create_restaurant_template() -> OmsResult<OmsDocument> {
    let metadata = Metadata {
        created: Utc::now(),
        source: "open_menu_standard".to_string(),
        locale: "en-US".to_string(),
    };
    
    let vendor = Vendor {
        id: "restaurant-template".to_string(),
        name: "Restaurant Template".to_string(),
        r#type: "restaurant".to_string(),
        location_id: None,
        location_name: None,
        address: None,
        contact: None,
        hours: None,
        cuisine: None,
        services: None,
    };
    
    // Create a customization for cooking preference
    let cooking_pref = Customization {
        id: "cooking-pref".to_string(),
        name: "Cooking Preference".to_string(),
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
                id: "rare".to_string(),
                name: "Rare".to_string(),
                price_adjustment: None,
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: None,
            },
            CustomizationOption {
                id: "medium-rare".to_string(),
                name: "Medium Rare".to_string(),
                price_adjustment: None,
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: None,
            },
            CustomizationOption {
                id: "medium".to_string(),
                name: "Medium".to_string(),
                price_adjustment: None,
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: None,
            },
            CustomizationOption {
                id: "medium-well".to_string(),
                name: "Medium Well".to_string(),
                price_adjustment: None,
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: None,
            },
            CustomizationOption {
                id: "well-done".to_string(),
                name: "Well Done".to_string(),
                price_adjustment: None,
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: None,
            },
        ]),
    };
    
    // Create a customization for sides
    let sides = Customization {
        id: "side".to_string(),
        name: "Side".to_string(),
        r#type: CustomizationType::SingleSelect,
        required: true,
        default: CustomizationDefault::String("fries".to_string()),
        min_selections: None,
        max_selections: None,
        min: None,
        max: None,
        step: None,
        unit_price_adjustment: None,
        unit_nutrition_adjustments: None,
        options: Some(vec![
            CustomizationOption {
                id: "fries".to_string(),
                name: "French Fries".to_string(),
                price_adjustment: None,
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: None,
            },
            CustomizationOption {
                id: "salad".to_string(),
                name: "House Salad".to_string(),
                price_adjustment: None,
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: None,
            },
            CustomizationOption {
                id: "soup".to_string(),
                name: "Soup of the Day".to_string(),
                price_adjustment: None,
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: None,
            },
        ]),
    };
    
    // Create an item
    let steak = Item {
        id: "steak".to_string(),
        name: "New York Strip Steak".to_string(),
        category: "entree".to_string(),
        vendor_id: None,
        description: Some("12oz New York Strip steak with choice of side".to_string()),
        subcategory: None,
        image_url: None,
        base_price: Some(29.99),
        currency: Some("USD".to_string()),
        nutrition: None,
        customizations: Some(vec![cooking_pref, sides]),
        selected_customizations: None,
        quantity: None,
        item_note: None,
        calculated: None,
        components: None,
        availability: None,
        popularity: None,
    };
    
    Ok(OmsDocument::new(metadata, vendor, vec![steak]))
}

/// Create a cafe template
fn create_cafe_template() -> OmsResult<OmsDocument> {
    let metadata = Metadata {
        created: Utc::now(),
        source: "open_menu_standard".to_string(),
        locale: "en-US".to_string(),
    };
    
    let vendor = Vendor {
        id: "cafe-template".to_string(),
        name: "Cafe Template".to_string(),
        r#type: "cafe".to_string(),
        location_id: None,
        location_name: None,
        address: None,
        contact: None,
        hours: None,
        cuisine: None,
        services: None,
    };
    
    // Create a customization for bread type
    let bread = Customization {
        id: "bread".to_string(),
        name: "Bread".to_string(),
        r#type: CustomizationType::SingleSelect,
        required: true,
        default: CustomizationDefault::String("wheat".to_string()),
        min_selections: None,
        max_selections: None,
        min: None,
        max: None,
        step: None,
        unit_price_adjustment: None,
        unit_nutrition_adjustments: None,
        options: Some(vec![
            CustomizationOption {
                id: "wheat".to_string(),
                name: "Wheat".to_string(),
                price_adjustment: None,
                nutrition_adjustments: None,
                allergens: Some(vec!["wheat".to_string()]),
                dietary_flags: None,
            },
            CustomizationOption {
                id: "white".to_string(),
                name: "White".to_string(),
                price_adjustment: None,
                nutrition_adjustments: None,
                allergens: Some(vec!["wheat".to_string()]),
                dietary_flags: None,
            },
            CustomizationOption {
                id: "rye".to_string(),
                name: "Rye".to_string(),
                price_adjustment: None,
                nutrition_adjustments: None,
                allergens: Some(vec!["wheat".to_string()]),
                dietary_flags: None,
            },
        ]),
    };
    
    // Create a customization for cheese
    let cheese = Customization {
        id: "cheese".to_string(),
        name: "Cheese".to_string(),
        r#type: CustomizationType::SingleSelect,
        required: false,
        default: CustomizationDefault::String("cheddar".to_string()),
        min_selections: None,
        max_selections: None,
        min: None,
        max: None,
        step: None,
        unit_price_adjustment: None,
        unit_nutrition_adjustments: None,
        options: Some(vec![
            CustomizationOption {
                id: "cheddar".to_string(),
                name: "Cheddar".to_string(),
                price_adjustment: None,
                nutrition_adjustments: None,
                allergens: Some(vec!["dairy".to_string()]),
                dietary_flags: None,
            },
            CustomizationOption {
                id: "swiss".to_string(),
                name: "Swiss".to_string(),
                price_adjustment: None,
                nutrition_adjustments: None,
                allergens: Some(vec!["dairy".to_string()]),
                dietary_flags: None,
            },
            CustomizationOption {
                id: "none".to_string(),
                name: "No Cheese".to_string(),
                price_adjustment: None,
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: Some(vec!["dairy_free".to_string()]),
            },
        ]),
    };
    
    // Create an item
    let sandwich = Item {
        id: "turkey-sandwich".to_string(),
        name: "Turkey Sandwich".to_string(),
        category: "sandwich".to_string(),
        vendor_id: None,
        description: Some("Roasted turkey breast with lettuce, tomato, and choice of cheese and bread".to_string()),
        subcategory: None,
        image_url: None,
        base_price: Some(8.99),
        currency: Some("USD".to_string()),
        nutrition: None,
        customizations: Some(vec![bread, cheese]),
        selected_customizations: None,
        quantity: None,
        item_note: None,
        calculated: None,
        components: None,
        availability: None,
        popularity: None,
    };
    
    Ok(OmsDocument::new(metadata, vendor, vec![sandwich]))
}

/// Create a fast-food template
fn create_fast_food_template() -> OmsResult<OmsDocument> {
    let metadata = Metadata {
        created: Utc::now(),
        source: "open_menu_standard".to_string(),
        locale: "en-US".to_string(),
    };
    
    let vendor = Vendor {
        id: "fast-food-template".to_string(),
        name: "Fast Food Template".to_string(),
        r#type: "fast-food".to_string(),
        location_id: None,
        location_name: None,
        address: None,
        contact: None,
        hours: None,
        cuisine: None,
        services: None,
    };
    
    // Create a combo meal with components
    let burger = Item {
        id: "burger".to_string(),
        name: "Cheeseburger".to_string(),
        category: "burger".to_string(),
        vendor_id: None,
        description: Some("Quarter-pound beef patty with cheese, lettuce, tomato, and special sauce".to_string()),
        subcategory: None,
        image_url: None,
        base_price: Some(4.99),
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
    
    // Create drink customization
    let drink = Customization {
        id: "drink".to_string(),
        name: "Drink".to_string(),
        r#type: CustomizationType::SingleSelect,
        required: true,
        default: CustomizationDefault::String("cola".to_string()),
        min_selections: None,
        max_selections: None,
        min: None,
        max: None,
        step: None,
        unit_price_adjustment: None,
        unit_nutrition_adjustments: None,
        options: Some(vec![
            CustomizationOption {
                id: "cola".to_string(),
                name: "Cola".to_string(),
                price_adjustment: None,
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: None,
            },
            CustomizationOption {
                id: "diet-cola".to_string(),
                name: "Diet Cola".to_string(),
                price_adjustment: None,
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: None,
            },
            CustomizationOption {
                id: "lemon-lime".to_string(),
                name: "Lemon-Lime Soda".to_string(),
                price_adjustment: None,
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: None,
            },
        ]),
    };
    
    // Create side customization
    let side = Customization {
        id: "side".to_string(),
        name: "Side".to_string(),
        r#type: CustomizationType::SingleSelect,
        required: true,
        default: CustomizationDefault::String("fries".to_string()),
        min_selections: None,
        max_selections: None,
        min: None,
        max: None,
        step: None,
        unit_price_adjustment: None,
        unit_nutrition_adjustments: None,
        options: Some(vec![
            CustomizationOption {
                id: "fries".to_string(),
                name: "French Fries".to_string(),
                price_adjustment: None,
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: None,
            },
            CustomizationOption {
                id: "onion-rings".to_string(),
                name: "Onion Rings".to_string(),
                price_adjustment: Some(1.00),
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: None,
            },
        ]),
    };
    
    // Create an item with components
    let combo = Item {
        id: "combo".to_string(),
        name: "Cheeseburger Combo".to_string(),
        category: "combo".to_string(),
        vendor_id: None,
        description: Some("Cheeseburger with fries and a drink".to_string()),
        subcategory: None,
        image_url: None,
        base_price: Some(7.99),
        currency: Some("USD".to_string()),
        nutrition: None,
        customizations: Some(vec![drink, side]),
        selected_customizations: None,
        quantity: None,
        item_note: None,
        calculated: None,
        components: Some(vec![burger]),
        availability: None,
        popularity: None,
    };
    
    Ok(OmsDocument::new(metadata, vendor, vec![combo]))
}

/// Create a coffee shop template
fn create_coffee_shop_template() -> OmsResult<OmsDocument> {
    let metadata = Metadata {
        created: Utc::now(),
        source: "open_menu_standard".to_string(),
        locale: "en-US".to_string(),
    };
    
    let vendor = Vendor {
        id: "coffee-shop-template".to_string(),
        name: "Coffee Shop Template".to_string(),
        r#type: "coffee-shop".to_string(),
        location_id: None,
        location_name: None,
        address: None,
        contact: None,
        hours: None,
        cuisine: None,
        services: None,
    };
    
    // Create size customization
    let size = Customization {
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
    let milk = Customization {
        id: "milk".to_string(),
        name: "Milk".to_string(),
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
                allergens: Some(vec!["gluten".to_string()]),
                dietary_flags: Some(vec!["dairy_free".to_string(), "vegan".to_string()]),
            },
        ]),
    };
    
    // Create espresso shots customization
    let shots = Customization {
        id: "shots".to_string(),
        name: "Espresso Shots".to_string(),
        r#type: CustomizationType::Quantity,
        required: true,
        default: CustomizationDefault::Number(2.0),
        min_selections: None,
        max_selections: None,
        min: Some(1.0),
        max: Some(5.0),
        step: Some(1.0),
        unit_price_adjustment: Some(0.75),
        unit_nutrition_adjustments: None,
        options: None,
    };
    
    // Create flavor customization
    let flavor = Customization {
        id: "flavor".to_string(),
        name: "Flavor Syrup".to_string(),
        r#type: CustomizationType::MultiSelect,
        required: false,
        default: CustomizationDefault::StringArray(vec![]),
        min_selections: Some(0),
        max_selections: Some(3),
        min: None,
        max: None,
        step: None,
        unit_price_adjustment: None,
        unit_nutrition_adjustments: None,
        options: Some(vec![
            CustomizationOption {
                id: "vanilla".to_string(),
                name: "Vanilla".to_string(),
                price_adjustment: Some(0.50),
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: None,
            },
            CustomizationOption {
                id: "caramel".to_string(),
                name: "Caramel".to_string(),
                price_adjustment: Some(0.50),
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: None,
            },
            CustomizationOption {
                id: "hazelnut".to_string(),
                name: "Hazelnut".to_string(),
                price_adjustment: Some(0.50),
                nutrition_adjustments: None,
                allergens: Some(vec!["tree-nuts".to_string()]),
                dietary_flags: None,
            },
        ]),
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
        nutrition: None,
        customizations: Some(vec![size.clone(), milk.clone(), shots.clone(), flavor.clone()]),
        selected_customizations: None,
        quantity: None,
        item_note: None,
        calculated: None,
        components: None,
        availability: None,
        popularity: None,
    };
    
    // Create cappuccino item
    let cappuccino = Item {
        id: "cappuccino".to_string(),
        name: "Cappuccino".to_string(),
        category: "coffee".to_string(),
        vendor_id: None,
        description: Some("Espresso with equal parts steamed milk and foamed milk".to_string()),
        subcategory: None,
        image_url: None,
        base_price: Some(4.25),
        currency: Some("USD".to_string()),
        nutrition: None,
        customizations: Some(vec![size, milk, shots, flavor]),
        selected_customizations: None,
        quantity: None,
        item_note: None,
        calculated: None,
        components: None,
        availability: None,
        popularity: None,
    };
    
    Ok(OmsDocument::new(metadata, vendor, vec![latte, cappuccino]))
}

/// Create a pizzeria template
fn create_pizzeria_template() -> OmsResult<OmsDocument> {
    let metadata = Metadata {
        created: Utc::now(),
        source: "open_menu_standard".to_string(),
        locale: "en-US".to_string(),
    };
    
    let vendor = Vendor {
        id: "pizzeria-template".to_string(),
        name: "Pizzeria Template".to_string(),
        r#type: "pizzeria".to_string(),
        location_id: None,
        location_name: None,
        address: None,
        contact: None,
        hours: None,
        cuisine: None,
        services: None,
    };
    
    // Create size customization
    let size = Customization {
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
                name: "Small (10\")".to_string(),
                price_adjustment: Some(-2.00),
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: None,
            },
            CustomizationOption {
                id: "medium".to_string(),
                name: "Medium (12\")".to_string(),
                price_adjustment: Some(0.0),
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: None,
            },
            CustomizationOption {
                id: "large".to_string(),
                name: "Large (14\")".to_string(),
                price_adjustment: Some(2.00),
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: None,
            },
            CustomizationOption {
                id: "x-large".to_string(),
                name: "X-Large (16\")".to_string(),
                price_adjustment: Some(4.00),
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: None,
            },
        ]),
    };
    
    // Create crust customization
    let crust = Customization {
        id: "crust".to_string(),
        name: "Crust".to_string(),
        r#type: CustomizationType::SingleSelect,
        required: true,
        default: CustomizationDefault::String("regular".to_string()),
        min_selections: None,
        max_selections: None,
        min: None,
        max: None,
        step: None,
        unit_price_adjustment: None,
        unit_nutrition_adjustments: None,
        options: Some(vec![
            CustomizationOption {
                id: "regular".to_string(),
                name: "Regular".to_string(),
                price_adjustment: Some(0.0),
                nutrition_adjustments: None,
                allergens: Some(vec!["wheat".to_string()]),
                dietary_flags: None,
            },
            CustomizationOption {
                id: "thin".to_string(),
                name: "Thin".to_string(),
                price_adjustment: Some(0.0),
                nutrition_adjustments: None,
                allergens: Some(vec!["wheat".to_string()]),
                dietary_flags: None,
            },
            CustomizationOption {
                id: "stuffed".to_string(),
                name: "Cheese-Stuffed".to_string(),
                price_adjustment: Some(2.50),
                nutrition_adjustments: None,
                allergens: Some(vec!["wheat".to_string(), "dairy".to_string()]),
                dietary_flags: None,
            },
            CustomizationOption {
                id: "gluten-free".to_string(),
                name: "Gluten-Free".to_string(),
                price_adjustment: Some(3.00),
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: Some(vec!["gluten_free".to_string()]),
            },
        ]),
    };
    
    // Create toppings customization
    let toppings = Customization {
        id: "toppings".to_string(),
        name: "Toppings".to_string(),
        r#type: CustomizationType::MultiSelect,
        required: false,
        default: CustomizationDefault::StringArray(vec![]),
        min_selections: Some(0),
        max_selections: Some(10),
        min: None,
        max: None,
        step: None,
        unit_price_adjustment: None,
        unit_nutrition_adjustments: None,
        options: Some(vec![
            CustomizationOption {
                id: "pepperoni".to_string(),
                name: "Pepperoni".to_string(),
                price_adjustment: Some(1.50),
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: None,
            },
            CustomizationOption {
                id: "sausage".to_string(),
                name: "Sausage".to_string(),
                price_adjustment: Some(1.50),
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: None,
            },
            CustomizationOption {
                id: "mushrooms".to_string(),
                name: "Mushrooms".to_string(),
                price_adjustment: Some(1.00),
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: Some(vec!["vegetarian".to_string()]),
            },
            CustomizationOption {
                id: "onions".to_string(),
                name: "Onions".to_string(),
                price_adjustment: Some(1.00),
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: Some(vec!["vegetarian".to_string()]),
            },
            CustomizationOption {
                id: "peppers".to_string(),
                name: "Bell Peppers".to_string(),
                price_adjustment: Some(1.00),
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: Some(vec!["vegetarian".to_string()]),
            },
            CustomizationOption {
                id: "olives".to_string(),
                name: "Black Olives".to_string(),
                price_adjustment: Some(1.00),
                nutrition_adjustments: None,
                allergens: None,
                dietary_flags: Some(vec!["vegetarian".to_string()]),
            },
        ]),
    };
    
    // Create pizza item
    let pizza = Item {
        id: "cheese-pizza".to_string(),
        name: "Cheese Pizza".to_string(),
        category: "pizza".to_string(),
        vendor_id: None,
        description: Some("Classic cheese pizza with tomato sauce and mozzarella".to_string()),
        subcategory: None,
        image_url: None,
        base_price: Some(12.99),
        currency: Some("USD".to_string()),
        nutrition: None,
        customizations: Some(vec![size, crust, toppings]),
        selected_customizations: None,
        quantity: None,
        item_note: None,
        calculated: None,
        components: None,
        availability: None,
        popularity: None,
    };
    
    Ok(OmsDocument::new(metadata, vendor, vec![pizza]))
}

/// Save an OMS document to a file
pub fn save_document_to_file(document: &OmsDocument, path: &Path) -> OmsResult<()> {
    let json = document.to_json()?;
    fs::write(path, json)?;
    Ok(())
}

/// Load an OMS document from a file
pub fn load_document_from_file(path: &Path) -> OmsResult<OmsDocument> {
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    parse_oms_document(&contents)
}

/// Calculate price adjustments for selected customizations
pub fn calculate_price_adjustments(
    item: &Item,
    selected: &[SelectedCustomization],
) -> OmsResult<f64> {
    let mut total_adjustment = 0.0;
    
    if let Some(customizations) = &item.customizations {
        // Create a map of customizations for easy lookup
        let customization_map: HashMap<&str, &Customization> = customizations
            .iter()
            .map(|c| (c.id.as_str(), c))
            .collect();
        
        // Process each selected customization
        for selection in selected {
            let customization = match customization_map.get(selection.customization_id.as_str()) {
                Some(c) => c,
                None => continue, // Skip unknown customizations
            };
            
            match &customization.r#type {
                CustomizationType::SingleSelect => {
                    if let CustomizationSelection::String(selected_id) = &selection.selection {
                        if let Some(options) = &customization.options {
                            for option in options {
                                if option.id == *selected_id {
                                    if let Some(price_adj) = option.price_adjustment {
                                        total_adjustment += price_adj;
                                    }
                                    break;
                                }
                            }
                        }
                    }
                },
                CustomizationType::MultiSelect => {
                    if let CustomizationSelection::StringArray(selected_ids) = &selection.selection {
                        if let Some(options) = &customization.options {
                            for selected_id in selected_ids {
                                for option in options {
                                    if option.id == *selected_id {
                                        if let Some(price_adj) = option.price_adjustment {
                                            total_adjustment += price_adj;
                                        }
                                        break;
                                    }
                                }
                            }
                        }
                    }
                },
                CustomizationType::Quantity => {
                    if let CustomizationSelection::Number(quantity) = selection.selection {
                        if let Some(unit_price_adj) = customization.unit_price_adjustment {
                            total_adjustment += unit_price_adj * quantity;
                        }
                    }
                },
                // Boolean, Text, and Range don't have price adjustments in this implementation
                _ => {},
            }
        }
    }
    
    Ok(total_adjustment)
}

/// Extract and update only the customization selections from an OMS URL
pub fn extract_and_update_selections(
    url: &str,
    document: &mut OmsDocument,
) -> OmsResult<()> {
    let params = parse_oms_url(url)?;
    
    // Check if there's a customization preset parameter
    if let Some(preset_id) = params.get("c") {
        // In a real implementation, you'd look up the preset in a database
        // For this example, we'll just add a simple selection
        if let Some(item) = document.items.first_mut() {
            if let Some(customizations) = &item.customizations {
                if !customizations.is_empty() {
                    // Get the first customization ID for demonstration
                    let first_customization_id = customizations[0].id.clone();
                    
                    // Create a selection based on the customization type
                    let selection = match customizations[0].r#type {
                        CustomizationType::SingleSelect => {
                            // Use the preset ID as the selected option
                            CustomizationSelection::String(preset_id.clone())
                        },
                        CustomizationType::MultiSelect => {
                            // Use the preset ID as one of the selected options
                            CustomizationSelection::StringArray(vec![preset_id.clone()])
                        },
                        CustomizationType::Quantity => {
                            // Try to parse the preset ID as a number
                            match preset_id.parse::<f64>() {
                                Ok(val) => CustomizationSelection::Number(val),
                                Err(_) => CustomizationSelection::Number(1.0), // Default to 1
                            }
                        },
                        CustomizationType::Boolean => {
                            // Try to parse the preset ID as a boolean
                            match preset_id.to_lowercase().as_str() {
                                "true" | "1" | "yes" => CustomizationSelection::Boolean(true),
                                _ => CustomizationSelection::Boolean(false),
                            }
                        },
                        CustomizationType::Text => {
                            // Use the preset ID as the text value
                            CustomizationSelection::String(preset_id.clone())
                        },
                        CustomizationType::Range => {
                            // Try to parse the preset ID as a number
                            match preset_id.parse::<f64>() {
                                Ok(val) => CustomizationSelection::Number(val),
                                Err(_) => CustomizationSelection::Number(0.0), // Default to 0
                            }
                        },
                    };
                    
                    // Create or update the selected_customizations array
                    let selected = item.selected_customizations.get_or_insert_with(Vec::new);
                    
                    // Check if this customization is already selected
                    let existing_idx = selected.iter().position(|s| s.customization_id == first_customization_id);
                    
                    if let Some(idx) = existing_idx {
                        // Update existing selection
                        selected[idx].selection = selection;
                    } else {
                        // Add new selection
                        selected.push(SelectedCustomization {
                            customization_id: first_customization_id,
                            selection,
                        });
                    }
                }
            }
        }
    }
    
    Ok(())
}

/// Generate a complete order from a document
pub fn generate_order(document: &mut OmsDocument, customer_id: Option<&str>) -> OmsResult<()> {
    // Calculate total price
    let subtotal = document.calculate_total_price().unwrap_or(0.0);
    let tax_rate = 0.08; // 8% tax rate
    let tax = (subtotal * tax_rate * 100.0).round() / 100.0; // Round to 2 decimal places
    let total = subtotal + tax;
    
    // Create an order
    let order = Order {
        id: Some(format!("order-{}", uuid::Uuid::new_v4())),
        status: Some(OrderStatus::Draft),
        created: Some(Utc::now()),
        pickup_time: Some(Utc::now() + chrono::Duration::minutes(30)),
        delivery_time: None,
        r#type: Some(OrderType::Pickup),
        customer_notes: None,
        payment: Some(Payment {
            status: Some(PaymentStatus::Unpaid),
            method: None,
            subtotal: Some(subtotal),
            tax: Some(tax),
            tip: None,
            total,
            currency: "USD".to_string(),
        }),
        customer: customer_id.map(|id| Customer {
            id: Some(id.to_string()),
            name: None,
            phone: None,
            email: None,
        }),
        delivery: None,
    };
    
    document.set_order(order);
    Ok(())
}

/// Check if an OMS document is a valid tap-to-order document
pub fn is_valid_tap_to_order(document: &OmsDocument) -> bool {
    // A valid tap-to-order document must have:
    // 1. A vendor with an ID
    // 2. At least one item
    // 3. Each item must have a base price
    
    if document.vendor.id.is_empty() {
        return false;
    }
    
    if document.items.is_empty() {
        return false;
    }
    
    for item in &document.items {
        if item.base_price.is_none() {
            return false;
        }
    }
    
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_create_minimal_document() {
        let doc = create_minimal_document(
            "test-vendor",
            "Test Restaurant",
            "restaurant",
            "test-item",
            "Test Burger",
            "burger",
        ).unwrap();
        
        assert_eq!(doc.vendor.id, "test-vendor");
        assert_eq!(doc.vendor.name, "Test Restaurant");
        assert_eq!(doc.items.len(), 1);
        assert_eq!(doc.items[0].name, "Test Burger");
    }
    
    #[test]
    fn test_create_template() {
        // Test restaurant template
        let restaurant = create_template("restaurant").unwrap();
        assert_eq!(restaurant.vendor.r#type, "restaurant");
        assert_eq!(restaurant.items.len(), 1);
        assert_eq!(restaurant.items[0].name, "New York Strip Steak");
        
        // Test coffee shop template
        let coffee_shop = create_template("coffee-shop").unwrap();
        assert_eq!(coffee_shop.vendor.r#type, "coffee-shop");
        assert_eq!(coffee_shop.items.len(), 2);
        assert_eq!(coffee_shop.items[0].name, "Latte");
        assert_eq!(coffee_shop.items[1].name, "Cappuccino");
        
        // Test invalid template
        let result = create_template("invalid");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_save_and_load_document() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.omenu");
        
        let doc = create_minimal_document(
            "test-vendor",
            "Test Restaurant",
            "restaurant",
            "test-item",
            "Test Burger",
            "burger",
        ).unwrap();
        
        // Save the document
        save_document_to_file(&doc, &file_path).unwrap();
        
        // Load the document
        let loaded_doc = load_document_from_file(&file_path).unwrap();
        
        assert_eq!(doc.vendor.id, loaded_doc.vendor.id);
        assert_eq!(doc.items[0].name, loaded_doc.items[0].name);
    }
    
    #[test]
    fn test_calculate_price_adjustments() {
        // Create an item with customizations
        let mut doc = create_template("coffee-shop").unwrap();
        let item = &doc.items[0]; // Latte
        
        // Create some selections
        let selections = vec![
            SelectedCustomization {
                customization_id: "size".to_string(),
                selection: CustomizationSelection::String("large".to_string()),
            },
            SelectedCustomization {
                customization_id: "milk".to_string(),
                selection: CustomizationSelection::String("almond".to_string()),
            },
            SelectedCustomization {
                customization_id: "shots".to_string(),
                selection: CustomizationSelection::Number(3.0),
            },
            SelectedCustomization {
                customization_id: "flavor".to_string(),
                selection: CustomizationSelection::StringArray(vec![
                    "vanilla".to_string(),
                    "caramel".to_string(),
                ]),
            },
        ];
        
        // Calculate price adjustments
        let adjustment = calculate_price_adjustments(item, &selections).unwrap();
        
        // Expected adjustment:
        // Size large: +0.50
        // Almond milk: +0.75
        // Extra shot (1): +0.75
        // Vanilla: +0.50
        // Caramel: +0.50
        // Total: +3.00
        assert_eq!(adjustment, 3.00);
    }
    
    #[test]
    fn test_extract_and_update_selections() {
        let mut doc = create_template("coffee-shop").unwrap();
        
        // Test URL with customization preset
        let url = "omenu://order?v=coffee-shop-template&i=latte&c=large";
        extract_and_update_selections(url, &mut doc).unwrap();
        
        // Verify that a selection was added
        let item = &doc.items[0]; // Latte
        assert!(item.selected_customizations.is_some());
        let selections = item.selected_customizations.as_ref().unwrap();
        assert_eq!(selections.len(), 1);
        assert_eq!(selections[0].customization_id, "size");
        
        match &selections[0].selection {
            CustomizationSelection::String(val) => assert_eq!(val, "large"),
            _ => panic!("Unexpected selection type"),
        }
    }
    
    #[test]
    fn test_generate_order() {
        let mut doc = create_minimal_document(
            "test-vendor",
            "Test Restaurant",
            "restaurant",
            "test-item",
            "Test Burger",
            "burger",
        ).unwrap();
        
        // Set a price for the item
        doc.items[0].base_price = Some(10.0);
        
        // Generate an order
        generate_order(&mut doc, Some("test-customer")).unwrap();
        
        // Verify the order
        assert!(doc.order.is_some());
        let order = doc.order.as_ref().unwrap();
        assert_eq!(order.r#type, Some(OrderType::Pickup));
        
        // Verify payment details
        let payment = order.payment.as_ref().unwrap();
        assert_eq!(payment.subtotal, Some(10.0));
        assert_eq!(payment.tax, Some(0.8)); // 8% of 10.0
        assert_eq!(payment.total, 10.8);
        
        // Verify customer
        let customer = order.customer.as_ref().unwrap();
        assert_eq!(customer.id, Some("test-customer".to_string()));
    }
    
    #[test]
    fn test_is_valid_tap_to_order() {
        // Valid document
        let mut doc = create_minimal_document(
            "test-vendor",
            "Test Restaurant",
            "restaurant",
            "test-item",
            "Test Burger",
            "burger",
        ).unwrap();
        
        doc.items[0].base_price = Some(10.0);
        assert!(is_valid_tap_to_order(&doc));
        
        // Invalid document: no base price
        let doc_no_price = create_minimal_document(
            "test-vendor",
            "Test Restaurant",
            "restaurant",
            "test-item",
            "Test Burger",
            "burger",
        ).unwrap();
        
        assert!(!is_valid_tap_to_order(&doc_no_price));
        
        // Invalid document: no items
        let mut doc_no_items = doc.clone();
        doc_no_items.items.clear();
        assert!(!is_valid_tap_to_order(&doc_no_items));
        
        // Invalid document: no vendor ID
        let mut doc_no_vendor_id = doc;
        doc_no_vendor_id.vendor.id = "".to_string();
        assert!(!is_valid_tap_to_order(&doc_no_vendor_id));
    }
}