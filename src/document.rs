// src/document.rs
//
// Implementation of OmsDocument methods

use std::collections::HashMap;

use crate::{OMS_VERSION, OmsError, OmsResult};
use crate::types::*;
use crate::validation::validate_document;
use chrono::Utc;
use serde_json::{to_string_pretty, from_str};
use validator::Validate;

impl OmsDocument {
    /// Create a new OMS document with the minimum required fields
    pub fn new(metadata: Metadata, vendor: Vendor, items: Vec<Item>) -> Self {
        Self {
            oms_version: OMS_VERSION.to_string(),
            metadata,
            vendor,
            items,
            order: None,
            extensions: None,
        }
    }
    
    /// Create a new OMS document with an order
    pub fn with_order(metadata: Metadata, vendor: Vendor, items: Vec<Item>, order: Order) -> Self {
        Self {
            oms_version: OMS_VERSION.to_string(),
            metadata,
            vendor,
            items,
            order: Some(order),
            extensions: None,
        }
    }
    
    /// Validate the OMS document according to the specification
    pub fn validate(&self) -> OmsResult<()> {
        // Perform additional validations
        validate_document(self)?;
        
        Ok(())
    }
    
    /// Serialize the OMS document to a JSON string
    pub fn to_json(&self) -> OmsResult<String> {
        let json = to_string_pretty(self)?;
        Ok(json)
    }
    
    /// Serialize the OMS document to a compact JSON string (for NFC tags)
    pub fn to_compact_json(&self) -> OmsResult<String> {
        let json = serde_json::to_string(self)?;
        Ok(json)
    }
    
    /// Deserialize an OMS document from a JSON string
    pub fn from_json(json: &str) -> OmsResult<Self> {
        let document: Self = from_str(json)?;
        document.validate()?;
        Ok(document)
    }
    
    /// Calculate total price for all items in the order
    pub fn calculate_total_price(&self) -> Option<f64> {
        // Sum up the prices of all items
        let items_total = self.items.iter().fold(0.0, |acc, item| {
            // Get the base price or fallback to 0.0
            let base_price = item.base_price.unwrap_or(0.0);
            
            // Get the quantity or fallback to 1
            let quantity = item.quantity.unwrap_or(1) as f64;
            
            // Get the calculated price if available
            let item_price = match &item.calculated {
                Some(calc) => calc.item_price,
                None => base_price,
            };
            
            acc + (item_price * quantity)
        });
        
        // Return the total if it's greater than zero
        if items_total > 0.0 {
            Some(items_total)
        } else {
            None
        }
    }
    
    /// Create an OMS URL for this document
    pub fn create_url(&self) -> Option<String> {
        // We need vendor ID to create a URL
        let vendor_id = &self.vendor.id;
        
        // Get the location ID if available
        let location_param = match &self.vendor.location_id {
            Some(location_id) => format!("&l={}", location_id),
            None => String::new(),
        };
        
        // Use the first item ID if available
        if let Some(first_item) = self.items.first() {
            let item_id = &first_item.id;
            Some(format!("omenu://order?v={}{}&i={}", vendor_id, location_param, item_id))
        } else {
            // If no items, just return the vendor URL
            Some(format!("omenu://view?v={}{}", vendor_id, location_param))
        }
    }
    
    /// Add an item to the document
    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }
    
    /// Remove an item by ID
    pub fn remove_item(&mut self, item_id: &str) -> bool {
        let initial_len = self.items.len();
        self.items.retain(|item| item.id != item_id);
        self.items.len() < initial_len
    }
    
    /// Find an item by ID
    pub fn find_item(&self, item_id: &str) -> Option<&Item> {
        self.items.iter().find(|item| item.id == item_id)
    }
    
    /// Find an item by ID and return a mutable reference
    pub fn find_item_mut(&mut self, item_id: &str) -> Option<&mut Item> {
        self.items.iter_mut().find(|item| item.id == item_id)
    }
    
    /// Add order information to the document
    pub fn set_order(&mut self, order: Order) {
        self.order = Some(order);
    }
    
    /// Update the order status
    pub fn update_order_status(&mut self, status: OrderStatus) -> OmsResult<()> {
        match &mut self.order {
            Some(order) => {
                order.status = Some(status);
                Ok(())
            },
            None => Err(OmsError::MissingRequiredField("order".to_string())),
        }
    }
    
    /// Extract selected customizations as a compact representation
    pub fn extract_customization_selections(&self) -> HashMap<String, Vec<SelectedCustomization>> {
        let mut result = HashMap::new();
        
        for item in &self.items {
            if let Some(selections) = &item.selected_customizations {
                result.insert(item.id.clone(), selections.clone());
            }
        }
        
        result
    }
    
    /// Add an extension to the document
    pub fn add_extension(&mut self, namespace: &str, data: serde_json::Value) {
        let extensions = self.extensions.get_or_insert_with(HashMap::new);
        extensions.insert(namespace.to_string(), data);
    }
    
    /// Get an extension from the document
    pub fn get_extension(&self, namespace: &str) -> Option<&serde_json::Value> {
        self.extensions.as_ref().and_then(|e| e.get(namespace))
    }
    
    /// Create a new OmsDocument with the current timestamp
    pub fn now(vendor_id: &str, vendor_name: &str, vendor_type: &str) -> Self {
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
        
        Self::new(metadata, vendor, Vec::new())
    }
}

/// Parse an OMS document from a JSON string
pub fn parse_oms_document(json: &str) -> OmsResult<OmsDocument> {
    OmsDocument::from_json(json)
}

/// Create a compact JSON representation suitable for NFC tags
pub fn create_compact_oms_json(document: &OmsDocument) -> OmsResult<String> {
    document.to_compact_json()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;
    
    fn create_test_document() -> OmsDocument {
        let metadata = Metadata {
            created: Utc::now(),
            source: "test".to_string(),
            locale: "en-US".to_string(),
        };
        
        let vendor = Vendor {
            id: "test-vendor".to_string(),
            name: "Test Restaurant".to_string(),
            r#type: "restaurant".to_string(),
            location_id: None,
            location_name: None,
            address: None,
            contact: None,
            hours: None,
            cuisine: None,
            services: None,
        };
        
        let item = Item {
            id: "test-item".to_string(),
            name: "Test Item".to_string(),
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
        };
        
        OmsDocument::new(metadata, vendor, vec![item])
    }
    
    #[test]
    fn test_serialization() {
        let doc = create_test_document();
        let json = doc.to_json().unwrap();
        let parsed_doc = OmsDocument::from_json(&json).unwrap();
        
        assert_eq!(doc.vendor.id, parsed_doc.vendor.id);
        assert_eq!(doc.items[0].name, parsed_doc.items[0].name);
    }
    
    #[test]
    fn test_calculate_total_price() {
        let mut doc = create_test_document();
        
        // Test with one item
        let total = doc.calculate_total_price().unwrap();
        assert_eq!(total, 10.0);
        
        // Add another item
        let item2 = Item {
            id: "test-item-2".to_string(),
            name: "Test Item 2".to_string(),
            category: "test".to_string(),
            vendor_id: None,
            description: None,
            subcategory: None,
            image_url: None,
            base_price: Some(5.0),
            currency: Some("USD".to_string()),
            nutrition: None,
            customizations: None,
            selected_customizations: None,
            quantity: Some(2),
            item_note: None,
            calculated: None,
            components: None,
            availability: None,
            popularity: None,
        };
        
        doc.add_item(item2);
        
        // Test with two items
        let total = doc.calculate_total_price().unwrap();
        assert_eq!(total, 10.0 + (5.0 * 2.0));
    }
    
    #[test]
    fn test_create_url() {
        let doc = create_test_document();
        let url = doc.create_url().unwrap();
        assert_eq!(url, "omenu://order?v=test-vendor&i=test-item");
    }
    
    #[test]
    fn test_find_item() {
        let doc = create_test_document();
        
        // Test finding existing item
        let item = doc.find_item("test-item").unwrap();
        assert_eq!(item.name, "Test Item");
        
        // Test finding non-existent item
        let item = doc.find_item("nonexistent");
        assert!(item.is_none());
    }
    
    #[test]
    fn test_remove_item() {
        let mut doc = create_test_document();
        
        // Add another item
        let item2 = Item {
            id: "test-item-2".to_string(),
            name: "Test Item 2".to_string(),
            category: "test".to_string(),
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
        
        doc.add_item(item2);
        assert_eq!(doc.items.len(), 2);
        
        // Remove an item
        let result = doc.remove_item("test-item");
        assert!(result);
        assert_eq!(doc.items.len(), 1);
        assert_eq!(doc.items[0].id, "test-item-2");
        
        // Try to remove a non-existent item
        let result = doc.remove_item("nonexistent");
        assert!(!result);
        assert_eq!(doc.items.len(), 1);
    }
    
    #[test]
    fn test_set_order() {
        let mut doc = create_test_document();
        
        let order = Order {
            id: Some("test-order".to_string()),
            status: Some(OrderStatus::Draft),
            created: Some(Utc::now()),
            pickup_time: None,
            delivery_time: None,
            r#type: Some(OrderType::Pickup),
            customer_notes: None,
            payment: None,
            customer: None,
            delivery: None,
        };
        
        doc.set_order(order);
        
        assert!(doc.order.is_some());
        assert_eq!(doc.order.as_ref().unwrap().id, Some("test-order".to_string()));
    }
    
    #[test]
    fn test_update_order_status() {
        let mut doc = create_test_document();
        
        // Test updating when no order exists
        let result = doc.update_order_status(OrderStatus::Confirmed);
        assert!(result.is_err());
        
        // Add an order and test updating
        let order = Order {
            id: Some("test-order".to_string()),
            status: Some(OrderStatus::Draft),
            created: Some(Utc::now()),
            pickup_time: None,
            delivery_time: None,
            r#type: Some(OrderType::Pickup),
            customer_notes: None,
            payment: None,
            customer: None,
            delivery: None,
        };
        
        doc.set_order(order);
        
        let result = doc.update_order_status(OrderStatus::Confirmed);
        assert!(result.is_ok());
        assert_eq!(
            doc.order.as_ref().unwrap().status,
            Some(OrderStatus::Confirmed)
        );
    }
    
    #[test]
    fn test_extensions() {
        let mut doc = create_test_document();
        
        // Test adding an extension
        let data = serde_json::json!({
            "key": "value",
            "number": 42
        });
        
        doc.add_extension("com.example.test", data.clone());
        
        // Test getting the extension
        let ext = doc.get_extension("com.example.test").unwrap();
        assert_eq!(ext, &data);
        
        // Test getting a non-existent extension
        let ext = doc.get_extension("nonexistent");
        assert!(ext.is_none());
    }
}