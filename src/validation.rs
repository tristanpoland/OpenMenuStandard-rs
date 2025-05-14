// src/validation.rs
//
// Validation functions for OMS documents

use crate::{OmsError, OmsResult};
use crate::types::*;
use validator::ValidationError;

/// Validates a complete OmsDocument
pub fn validate_document(document: &OmsDocument) -> OmsResult<()> {
    // Check that at least one item exists
    if document.items.is_empty() {
        return Err(OmsError::ValidationError(validator::ValidationErrors::new()));
    }
    
    // Validate each item's customizations
    for item in &document.items {
        if let Some(customizations) = &item.customizations {
            validate_customizations(customizations)?;
        }
        
        // Validate selected customizations against available customizations
        if let Some(selected) = &item.selected_customizations {
            if let Some(available) = &item.customizations {
                validate_selected_customizations(selected, available)?;
            } else {
                return Err(OmsError::ValidationError(validator::ValidationErrors::new()));
            }
        }
    }
    
    // If order exists, validate it
    if let Some(order) = &document.order {
        validate_order(order, &document.items)?;
    }
    
    Ok(())
}

/// Validates customization definitions
fn validate_customizations(customizations: &[Customization]) -> OmsResult<()> {
    for customization in customizations {
        match customization.r#type {
            CustomizationType::SingleSelect | CustomizationType::MultiSelect => {
                // Options are required for select types
                if customization.options.is_none() || customization.options.as_ref().unwrap().is_empty() {
                    return Err(OmsError::MissingRequiredField(format!("options for customization {}", customization.id)));
                }
                
                // Validate default values
                match &customization.r#type {
                    CustomizationType::SingleSelect => {
                        match &customization.default {
                            CustomizationDefault::String(default_id) => {
                                // Check that default exists in options
                                let options = customization.options.as_ref().unwrap();
                                if !options.iter().any(|opt| &opt.id == default_id) {
                                    return Err(OmsError::InvalidFieldValue(format!(
                                        "default value '{}' not found in options for customization {}",
                                        default_id, customization.id
                                    )));
                                }
                            },
                            _ => return Err(OmsError::InvalidFieldValue(format!(
                                "default value type mismatch for single_select customization {}", 
                                customization.id
                            ))),
                        }
                    },
                    CustomizationType::MultiSelect => {
                        match &customization.default {
                            CustomizationDefault::StringArray(default_ids) => {
                                // Check that defaults exist in options
                                let options = customization.options.as_ref().unwrap();
                                for default_id in default_ids {
                                    if !options.iter().any(|opt| &opt.id == default_id) {
                                        return Err(OmsError::InvalidFieldValue(format!(
                                            "default value '{}' not found in options for customization {}",
                                            default_id, customization.id
                                        )));
                                    }
                                }
                                
                                // Check min/max selections
                                if let Some(min) = customization.min_selections {
                                    if default_ids.len() < min as usize {
                                        return Err(OmsError::InvalidFieldValue(format!(
                                            "default selections count is less than min_selections for customization {}", 
                                            customization.id
                                        )));
                                    }
                                }
                                
                                if let Some(max) = customization.max_selections {
                                    if default_ids.len() > max as usize {
                                        return Err(OmsError::InvalidFieldValue(format!(
                                            "default selections count is greater than max_selections for customization {}", 
                                            customization.id
                                        )));
                                    }
                                }
                            },
                            _ => return Err(OmsError::InvalidFieldValue(format!(
                                "default value type mismatch for multi_select customization {}", 
                                customization.id
                            ))),
                        }
                    },
                    _ => unreachable!(),
                }
            },
            CustomizationType::Quantity => {
                // Validate default is a number
                match customization.default {
                    CustomizationDefault::Number(value) => {
                        // Check min/max constraints
                        if let Some(min) = customization.min {
                            if value < min {
                                return Err(OmsError::InvalidFieldValue(format!(
                                    "default value {} is less than min {} for customization {}", 
                                    value, min, customization.id
                                )));
                            }
                        }
                        
                        if let Some(max) = customization.max {
                            if value > max {
                                return Err(OmsError::InvalidFieldValue(format!(
                                    "default value {} is greater than max {} for customization {}", 
                                    value, max, customization.id
                                )));
                            }
                        }
                    },
                    _ => return Err(OmsError::InvalidFieldValue(format!(
                        "default value type mismatch for quantity customization {}", 
                        customization.id
                    ))),
                }
            },
            CustomizationType::Boolean => {
                // Validate default is a boolean
                match customization.default {
                    CustomizationDefault::Boolean(_) => (), // Valid
                    _ => return Err(OmsError::InvalidFieldValue(format!(
                        "default value type mismatch for boolean customization {}", 
                        customization.id
                    ))),
                }
            },
            CustomizationType::Text => {
                // Validate default is a string
                match customization.default {
                    CustomizationDefault::String(_) => (), // Valid
                    _ => return Err(OmsError::InvalidFieldValue(format!(
                        "default value type mismatch for text customization {}", 
                        customization.id
                    ))),
                }
            },
            CustomizationType::Range => {
                // Validate default is a number
                match customization.default {
                    CustomizationDefault::Number(value) => {
                        // Check min/max constraints
                        if let Some(min) = customization.min {
                            if value < min {
                                return Err(OmsError::InvalidFieldValue(format!(
                                    "default value {} is less than min {} for customization {}", 
                                    value, min, customization.id
                                )));
                            }
                        }
                        
                        if let Some(max) = customization.max {
                            if value > max {
                                return Err(OmsError::InvalidFieldValue(format!(
                                    "default value {} is greater than max {} for customization {}", 
                                    value, max, customization.id
                                )));
                            }
                        }
                    },
                    _ => return Err(OmsError::InvalidFieldValue(format!(
                        "default value type mismatch for range customization {}", 
                        customization.id
                    ))),
                }
            },
        }
    }
    
    Ok(())
}

/// Validates selected customizations against available customizations
fn validate_selected_customizations(
    selected: &[SelectedCustomization],
    available: &[Customization]
) -> OmsResult<()> {
    // Build a map of available customizations for quick lookup
    let mut avail_map = std::collections::HashMap::new();
    for customization in available {
        avail_map.insert(&customization.id, customization);
    }
    
    // Check that all required customizations are selected
    for customization in available {
        if customization.required {
            if !selected.iter().any(|sel| sel.customization_id == customization.id) {
                return Err(OmsError::MissingRequiredField(format!(
                    "required customization {} not selected", 
                    customization.id
                )));
            }
        }
    }
    
    // Validate each selection
    for selection in selected {
        // Check that the customization exists
        let customization = match avail_map.get(&selection.customization_id) {
            Some(c) => c,
            None => return Err(OmsError::InvalidFieldValue(format!(
                "selected customization {} not found in available customizations", 
                selection.customization_id
            ))),
        };
        
        // Validate the selection based on customization type
        match customization.r#type {
            CustomizationType::SingleSelect => {
                match &selection.selection {
                    CustomizationSelection::String(selected_id) => {
                        // Check that the selection exists in options
                        let options = customization.options.as_ref().unwrap();
                        if !options.iter().any(|opt| &opt.id == selected_id) {
                            return Err(OmsError::InvalidFieldValue(format!(
                                "selected value '{}' not found in options for customization {}",
                                selected_id, customization.id
                            )));
                        }
                    },
                    _ => return Err(OmsError::InvalidFieldValue(format!(
                        "selection type mismatch for single_select customization {}", 
                        customization.id
                    ))),
                }
            },
            CustomizationType::MultiSelect => {
                match &selection.selection {
                    CustomizationSelection::StringArray(selected_ids) => {
                        // Check that selections exist in options
                        let options = customization.options.as_ref().unwrap();
                        for selected_id in selected_ids {
                            if !options.iter().any(|opt| &opt.id == selected_id) {
                                return Err(OmsError::InvalidFieldValue(format!(
                                    "selected value '{}' not found in options for customization {}",
                                    selected_id, customization.id
                                )));
                            }
                        }
                        
                        // Check min/max selections
                        if let Some(min) = customization.min_selections {
                            if selected_ids.len() < min as usize {
                                return Err(OmsError::InvalidFieldValue(format!(
                                    "selections count is less than min_selections for customization {}", 
                                    customization.id
                                )));
                            }
                        }
                        
                        if let Some(max) = customization.max_selections {
                            if selected_ids.len() > max as usize {
                                return Err(OmsError::InvalidFieldValue(format!(
                                    "selections count is greater than max_selections for customization {}", 
                                    customization.id
                                )));
                            }
                        }
                    },
                    _ => return Err(OmsError::InvalidFieldValue(format!(
                        "selection type mismatch for multi_select customization {}", 
                        customization.id
                    ))),
                }
            },
            CustomizationType::Quantity => {
                match selection.selection {
                    CustomizationSelection::Number(value) => {
                        // Check min/max constraints
                        if let Some(min) = customization.min {
                            if value < min {
                                return Err(OmsError::InvalidFieldValue(format!(
                                    "selected value {} is less than min {} for customization {}", 
                                    value, min, customization.id
                                )));
                            }
                        }
                        
                        if let Some(max) = customization.max {
                            if value > max {
                                return Err(OmsError::InvalidFieldValue(format!(
                                    "selected value {} is greater than max {} for customization {}", 
                                    value, max, customization.id
                                )));
                            }
                        }
                    },
                    _ => return Err(OmsError::InvalidFieldValue(format!(
                        "selection type mismatch for quantity customization {}", 
                        customization.id
                    ))),
                }
            },
            CustomizationType::Boolean => {
                match selection.selection {
                    CustomizationSelection::Boolean(_) => (), // Valid
                    _ => return Err(OmsError::InvalidFieldValue(format!(
                        "selection type mismatch for boolean customization {}", 
                        customization.id
                    ))),
                }
            },
            CustomizationType::Text => {
                match &selection.selection {
                    CustomizationSelection::String(_) => (), // Valid
                    _ => return Err(OmsError::InvalidFieldValue(format!(
                        "selection type mismatch for text customization {}", 
                        customization.id
                    ))),
                }
            },
            CustomizationType::Range => {
                match selection.selection {
                    CustomizationSelection::Number(value) => {
                        // Check min/max constraints
                        if let Some(min) = customization.min {
                            if value < min {
                                return Err(OmsError::InvalidFieldValue(format!(
                                    "selected value {} is less than min {} for customization {}", 
                                    value, min, customization.id
                                )));
                            }
                        }
                        
                        if let Some(max) = customization.max {
                            if value > max {
                                return Err(OmsError::InvalidFieldValue(format!(
                                    "selected value {} is greater than max {} for customization {}", 
                                    value, max, customization.id
                                )));
                            }
                        }
                    },
                    _ => return Err(OmsError::InvalidFieldValue(format!(
                        "selection type mismatch for range customization {}", 
                        customization.id
                    ))),
                }
            },
        }
    }
    
    Ok(())
}

/// Validates order information
fn validate_order(order: &Order, items: &[Item]) -> OmsResult<()> {
    // Check that there are items in the order
    if items.is_empty() {
        return Err(OmsError::ValidationError(validator::ValidationErrors::new()));
    }
    
    // Validate payment information
    if let Some(payment) = &order.payment {
        // Check that total is greater than zero
        if payment.total <= 0.0 {
            return Err(OmsError::InvalidFieldValue("payment total must be greater than zero".to_string()));
        }
        
        // If subtotal, tax, and tip are all provided, check that they add up to total
        if let (Some(subtotal), Some(tax), Some(tip)) = (payment.subtotal, payment.tax, payment.tip) {
            let calculated_total = subtotal + tax + tip;
            let epsilon = 0.01; // Allow for small floating-point errors
            
            if (calculated_total - payment.total).abs() > epsilon {
                return Err(OmsError::InvalidFieldValue(format!(
                    "payment components (subtotal + tax + tip = {}) do not add up to total ({})",
                    calculated_total, payment.total
                )));
            }
        }
    }
    
    // Validate delivery information
    if let Some(delivery) = &order.delivery {
        // If delivery type is specified, it should be "delivery"
        if let Some(order_type) = &order.r#type {
            if *order_type != OrderType::Delivery {
                return Err(OmsError::InvalidFieldValue(
                    "order.type must be 'delivery' when delivery information is provided".to_string()
                ));
            }
        }
    }
    
    // If order type is "delivery", delivery information should be provided
    if let Some(OrderType::Delivery) = &order.r#type {
        if order.delivery.is_none() {
            return Err(OmsError::MissingRequiredField(
                "delivery information is required for delivery orders".to_string()
            ));
        }
    }
    
    Ok(())
}

/// Validation function for customization type
pub fn validate_customization_type(type_str: &str) -> Result<(), ValidationError> {
    let valid_types = [
        "single_select", "multi_select", "quantity", "boolean", "text", "range",
    ];
    
    if valid_types.contains(&type_str) {
        Ok(())
    } else {
        let mut error = ValidationError::new("invalid_customization_type");
        error.message = Some(format!("Invalid customization type: {}. Must be one of: {}",
            type_str, valid_types.join(", ")).into());
        Err(error)
    }
}

/// Validation function for vendor type
pub fn validate_vendor_type(type_str: &str) -> Result<(), ValidationError> {
    let valid_types = [
        "restaurant", "cafe", "fast-food", "coffee-shop", "bakery", "grocery",
        "food-truck", "catering", "pizzeria", "pub", "bar",
    ];
    
    if valid_types.contains(&type_str) || !type_str.is_empty() {
        Ok(())
    } else {
        let mut error = ValidationError::new("invalid_vendor_type");
        error.message = Some(format!("Invalid vendor type: {}. Common types include: {}",
            type_str, valid_types.join(", ")).into());
        Err(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;
    
    #[test]
    fn test_validate_empty_document() {
        // Create a document with no items
        let doc = OmsDocument {
            oms_version: "1.0".to_string(),
            metadata: Metadata {
                created: chrono::Utc::now(),
                source: "test".to_string(),
                locale: "en-US".to_string(),
            },
            vendor: Vendor {
                id: "test".to_string(),
                name: "Test Vendor".to_string(),
                r#type: "restaurant".to_string(),
                location_id: None,
                location_name: None,
                address: None,
                contact: None,
                hours: None,
                cuisine: None,
                services: None,
            },
            items: vec![],
            order: None,
            extensions: None,
        };
        
        // Validation should fail
        let result = validate_document(&doc);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_validate_customizations() {
        // Valid single_select customization
        let single_select = Customization {
            id: "test-single".to_string(),
            name: "Test Single".to_string(),
            r#type: CustomizationType::SingleSelect,
            required: true,
            default: CustomizationDefault::String("option1".to_string()),
            min_selections: None,
            max_selections: None,
            min: None,
            max: None,
            step: None,
            unit_price_adjustment: None,
            unit_nutrition_adjustments: None,
            options: Some(vec![
                CustomizationOption {
                    id: "option1".to_string(),
                    name: "Option 1".to_string(),
                    price_adjustment: None,
                    nutrition_adjustments: None,
                    allergens: None,
                    dietary_flags: None,
                },
                CustomizationOption {
                    id: "option2".to_string(),
                    name: "Option 2".to_string(),
                    price_adjustment: None,
                    nutrition_adjustments: None,
                    allergens: None,
                    dietary_flags: None,
                },
            ]),
        };
        
        // Test valid customization
        let result = validate_customizations(&[single_select.clone()]);
        assert!(result.is_ok());
        
        // Test invalid default value
        let mut invalid_default = single_select.clone();
        invalid_default.default = CustomizationDefault::String("nonexistent".to_string());
        let result = validate_customizations(&[invalid_default]);
        assert!(result.is_err());
        
        // Test invalid default type
        let mut invalid_type = single_select.clone();
        invalid_type.default = CustomizationDefault::Number(1.0);
        let result = validate_customizations(&[invalid_type]);
        assert!(result.is_err());
        
        // Test missing options
        let mut missing_options = single_select;
        missing_options.options = None;
        let result = validate_customizations(&[missing_options]);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_validate_selected_customizations() {
        // Available customizations
        let customizations = vec![
            Customization {
                id: "test-single".to_string(),
                name: "Test Single".to_string(),
                r#type: CustomizationType::SingleSelect,
                required: true,
                default: CustomizationDefault::String("option1".to_string()),
                min_selections: None,
                max_selections: None,
                min: None,
                max: None,
                step: None,
                unit_price_adjustment: None,
                unit_nutrition_adjustments: None,
                options: Some(vec![
                    CustomizationOption {
                        id: "option1".to_string(),
                        name: "Option 1".to_string(),
                        price_adjustment: None,
                        nutrition_adjustments: None,
                        allergens: None,
                        dietary_flags: None,
                    },
                    CustomizationOption {
                        id: "option2".to_string(),
                        name: "Option 2".to_string(),
                        price_adjustment: None,
                        nutrition_adjustments: None,
                        allergens: None,
                        dietary_flags: None,
                    },
                ]),
            },
            Customization {
                id: "test-multi".to_string(),
                name: "Test Multi".to_string(),
                r#type: CustomizationType::MultiSelect,
                required: false,
                default: CustomizationDefault::StringArray(vec!["option1".to_string()]),
                min_selections: Some(0),
                max_selections: Some(2),
                min: None,
                max: None,
                step: None,
                unit_price_adjustment: None,
                unit_nutrition_adjustments: None,
                options: Some(vec![
                    CustomizationOption {
                        id: "option1".to_string(),
                        name: "Option 1".to_string(),
                        price_adjustment: None,
                        nutrition_adjustments: None,
                        allergens: None,
                        dietary_flags: None,
                    },
                    CustomizationOption {
                        id: "option2".to_string(),
                        name: "Option 2".to_string(),
                        price_adjustment: None,
                        nutrition_adjustments: None,
                        allergens: None,
                        dietary_flags: None,
                    },
                ]),
            },
        ];
        
        // Valid selections
        let selections = vec![
            SelectedCustomization {
                customization_id: "test-single".to_string(),
                selection: CustomizationSelection::String("option2".to_string()),
            },
            SelectedCustomization {
                customization_id: "test-multi".to_string(),
                selection: CustomizationSelection::StringArray(vec!["option1".to_string(), "option2".to_string()]),
            },
        ];
        
        // Test valid selections
        let result = validate_selected_customizations(&selections, &customizations);
        assert!(result.is_ok());
        
        // Test missing required customization
        let missing_required = vec![
            SelectedCustomization {
                customization_id: "test-multi".to_string(),
                selection: CustomizationSelection::StringArray(vec!["option1".to_string()]),
            },
        ];
        let result = validate_selected_customizations(&missing_required, &customizations);
        assert!(result.is_err());
        
        // Test invalid selection value
        let invalid_selection = vec![
            SelectedCustomization {
                customization_id: "test-single".to_string(),
                selection: CustomizationSelection::String("nonexistent".to_string()),
            },
        ];
        let result = validate_selected_customizations(&invalid_selection, &customizations);
        assert!(result.is_err());
        
        // Test invalid selection type
        let invalid_type = vec![
            SelectedCustomization {
                customization_id: "test-single".to_string(),
                selection: CustomizationSelection::Number(1.0),
            },
        ];
        let result = validate_selected_customizations(&invalid_type, &customizations);
        assert!(result.is_err());
        
        // Test nonexistent customization
        let nonexistent = vec![
            SelectedCustomization {
                customization_id: "nonexistent".to_string(),
                selection: CustomizationSelection::String("option1".to_string()),
            },
        ];
        let result = validate_selected_customizations(&nonexistent, &customizations);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_validate_order() {
        // Create items for the order
        let items = vec![
            Item {
                id: "item1".to_string(),
                name: "Item 1".to_string(),
                category: "test".to_string(),
                vendor_id: None,
                description: None,
                subcategory: None,
                image_url: None,
                base_price: Some(10.0),
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
            },
        ];
        
        // Valid order
        let order = Order {
            id: Some("order1".to_string()),
            status: Some(OrderStatus::Draft),
            created: Some(chrono::Utc::now()),
            pickup_time: None,
            delivery_time: None,
            r#type: Some(OrderType::Pickup),
            customer_notes: None,
            payment: Some(Payment {
                status: Some(PaymentStatus::Unpaid),
                method: None,
                subtotal: Some(10.0),
                tax: Some(0.8),
                tip: Some(2.0),
                total: 12.8,
                currency: "USD".to_string(),
            }),
            customer: None,
            delivery: None,
        };
        
        // Test valid order
        let result = validate_order(&order, &items);
        assert!(result.is_ok());
        
        // Test invalid payment total
        let mut invalid_total = order.clone();
        if let Some(payment) = &mut invalid_total.payment {
            payment.total = 0.0;
        }
        let result = validate_order(&invalid_total, &items);
        assert!(result.is_err());
        
        // Test inconsistent payment components
        let mut inconsistent = order.clone();
        if let Some(payment) = &mut inconsistent.payment {
            payment.total = 15.0; // Doesn't match subtotal + tax + tip
        }
        let result = validate_order(&inconsistent, &items);
        assert!(result.is_err());
        
        // Test delivery order without delivery info
        let mut missing_delivery = order.clone();
        missing_delivery.r#type = Some(OrderType::Delivery);
        let result = validate_order(&missing_delivery, &items);
        assert!(result.is_err());
        
        // Test valid delivery order
        let mut valid_delivery = order;
        valid_delivery.r#type = Some(OrderType::Delivery);
        valid_delivery.delivery = Some(Delivery {
            address: Address {
                street: "123 Main St".to_string(),
                city: "Anytown".to_string(),
                region: "State".to_string(),
                postal_code: "12345".to_string(),
                country: "USA".to_string(),
            },
            instructions: None,
        });
        let result = validate_order(&valid_delivery, &items);
        assert!(result.is_ok());
    }
}