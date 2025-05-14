// src/url.rs
//
// Functions for working with OMS URLs

use crate::{OMS_URL_SCHEME, OmsError, OmsResult};
use crate::types::OmsDocument;
use std::collections::HashMap;
use url::Url;

/// Parse an OMS URL and extract the parameters
pub fn parse_oms_url(url: &str) -> OmsResult<HashMap<String, String>> {
    if !url.starts_with(OMS_URL_SCHEME) {
        return Err(OmsError::InvalidOmsUrl(format!("URL must start with {}", OMS_URL_SCHEME)));
    }
    
    // Parse the URL manually to extract the action
    let without_scheme = url.strip_prefix(OMS_URL_SCHEME).unwrap_or("");
    let parts: Vec<&str> = without_scheme.split('?').collect();
    let action = parts[0];
    
    // Create the result map
    let mut params = HashMap::new();
    params.insert("action".to_string(), action.to_string());
    
    // Parse the URL for query parameters
    let url_obj = Url::parse(&format!("http://example.com/{}", without_scheme))
        .map_err(|e| OmsError::InvalidOmsUrl(format!("Failed to parse OMS URL: {}", e)))?;
    
    // Extract query parameters
    for (key, value) in url_obj.query_pairs() {
        params.insert(key.to_string(), value.to_string());
    }
    
    Ok(params)
}

/// Create an OMS URL from components
pub fn create_oms_url(
    action: &str,
    vendor_id: &str,
    location_id: Option<&str>,
    item_id: Option<&str>,
    customization_id: Option<&str>,
) -> OmsResult<String> {
    // Start with the scheme and action
    let mut url = format!("{}{}?v={}", OMS_URL_SCHEME, action, vendor_id);
    
    // Add optional parameters
    if let Some(location) = location_id {
        url.push_str(&format!("&l={}", location));
    }
    
    if let Some(item) = item_id {
        url.push_str(&format!("&i={}", item));
    }
    
    if let Some(customization) = customization_id {
        url.push_str(&format!("&c={}", customization));
    }
    
    Ok(url)
}

/// Create a view URL for a vendor
pub fn create_vendor_url(vendor_id: &str, location_id: Option<&str>) -> OmsResult<String> {
    create_oms_url("view", vendor_id, location_id, None, None)
}

/// Create an order URL for an item
pub fn create_order_url(
    vendor_id: &str, 
    item_id: &str, 
    location_id: Option<&str>,
    customization_id: Option<&str>,
) -> OmsResult<String> {
    create_oms_url("order", vendor_id, location_id, Some(item_id), customization_id)
}

/// Create a customize URL for an item
pub fn create_customize_url(
    vendor_id: &str, 
    item_id: &str, 
    location_id: Option<&str>,
) -> OmsResult<String> {
    create_oms_url("customize", vendor_id, location_id, Some(item_id), None)
}

/// Create a share URL for an item or document
pub fn create_share_url(
    vendor_id: &str, 
    item_id: Option<&str>, 
    location_id: Option<&str>,
) -> OmsResult<String> {
    create_oms_url("share", vendor_id, location_id, item_id, None)
}

/// Create a deep link to a document
pub fn create_deep_link(document: &OmsDocument) -> OmsResult<String> {
    // We need vendor ID to create a URL
    let vendor_id = &document.vendor.id;
    
    // Get the location ID if available
    let location_id = document.vendor.location_id.as_deref();
    
    // Use the first item ID if available
    if let Some(first_item) = document.items.first() {
        let item_id = &first_item.id;
        create_order_url(vendor_id, item_id, location_id, None)
    } else {
        // If no items, just return the vendor URL
        create_vendor_url(vendor_id, location_id)
    }
}

/// Add custom parameters to an OMS URL
pub fn add_custom_params(url: &str, params: &HashMap<String, String>) -> OmsResult<String> {
    if !url.starts_with(OMS_URL_SCHEME) {
        return Err(OmsError::InvalidOmsUrl(format!("URL must start with {}", OMS_URL_SCHEME)));
    }
    
    let mut result = url.to_string();
    
    for (key, value) in params {
        result.push_str(&format!("&{}={}", key, value));
    }
    
    Ok(result)
}

/// Encode a complete OMS document as a base64 URL parameter
#[cfg(feature = "network")]
pub fn encode_document_as_param(document: &OmsDocument) -> OmsResult<String> {
    let json = document.to_compact_json()?;
    let encoded = base64::encode(json);
    Ok(encoded)
}

/// Decode a base64-encoded OMS document from a URL parameter
#[cfg(feature = "network")]
pub fn decode_document_from_param(encoded: &str) -> OmsResult<OmsDocument> {
    let json = base64::decode(encoded)
        .map_err(|_| OmsError::InvalidFieldValue("Invalid base64 encoding".to_string()))?;
    
    let json_str = String::from_utf8(json)
        .map_err(|_| OmsError::InvalidFieldValue("Invalid UTF-8 encoding".to_string()))?;
    
    OmsDocument::from_json(&json_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_oms_url() {
        // Test a simple URL
        let url = "omenu://order?v=test-vendor&i=test-item";
        let params = parse_oms_url(url).unwrap();
        
        assert_eq!(params.get("action").unwrap(), "order");
        assert_eq!(params.get("v").unwrap(), "test-vendor");
        assert_eq!(params.get("i").unwrap(), "test-item");
        
        // Test a URL with more parameters
        let url = "omenu://customize?v=test-vendor&l=location-1&i=test-item&c=preset-1";
        let params = parse_oms_url(url).unwrap();
        
        assert_eq!(params.get("action").unwrap(), "customize");
        assert_eq!(params.get("v").unwrap(), "test-vendor");
        assert_eq!(params.get("l").unwrap(), "location-1");
        assert_eq!(params.get("i").unwrap(), "test-item");
        assert_eq!(params.get("c").unwrap(), "preset-1");
        
        // Test an invalid URL
        let url = "https://example.com/";
        let result = parse_oms_url(url);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_create_oms_url() {
        // Test with minimal parameters
        let url = create_oms_url("view", "test-vendor", None, None, None).unwrap();
        assert_eq!(url, "omenu://view?v=test-vendor");
        
        // Test with all parameters
        let url = create_oms_url(
            "order", 
            "test-vendor", 
            Some("location-1"), 
            Some("test-item"),
            Some("preset-1")
        ).unwrap();
        
        assert_eq!(url, "omenu://order?v=test-vendor&l=location-1&i=test-item&c=preset-1");
    }
    
    #[test]
    fn test_helper_functions() {
        // Test vendor URL
        let url = create_vendor_url("test-vendor", None).unwrap();
        assert_eq!(url, "omenu://view?v=test-vendor");
        
        // Test order URL
        let url = create_order_url("test-vendor", "test-item", None, None).unwrap();
        assert_eq!(url, "omenu://order?v=test-vendor&i=test-item");
        
        // Test customize URL
        let url = create_customize_url("test-vendor", "test-item", Some("location-1")).unwrap();
        assert_eq!(url, "omenu://customize?v=test-vendor&l=location-1&i=test-item");
        
        // Test share URL
        let url = create_share_url("test-vendor", Some("test-item"), None).unwrap();
        assert_eq!(url, "omenu://share?v=test-vendor&i=test-item");
    }
    
    #[test]
    fn test_add_custom_params() {
        let url = "omenu://order?v=test-vendor&i=test-item";
        let mut params = HashMap::new();
        params.insert("special".to_string(), "yes".to_string());
        params.insert("request".to_string(), "extra-sauce".to_string());
        
        let result = add_custom_params(url, &params).unwrap();
        
        // Note: order of parameters is not guaranteed, so we need to parse and check
        let parsed = parse_oms_url(&result).unwrap();
        assert_eq!(parsed.get("action").unwrap(), "order");
        assert_eq!(parsed.get("v").unwrap(), "test-vendor");
        assert_eq!(parsed.get("i").unwrap(), "test-item");
        assert_eq!(parsed.get("special").unwrap(), "yes");
        assert_eq!(parsed.get("request").unwrap(), "extra-sauce");
    }
    
    #[cfg(feature = "network")]
    #[test]
    fn test_encode_decode_document() {
        use crate::types::{Metadata, Vendor, Item};
        use chrono::Utc;
        
        // Create a simple document
        let doc = OmsDocument::new(
            Metadata {
                created: Utc::now(),
                source: "test".to_string(),
                locale: "en-US".to_string(),
            },
            Vendor {
                id: "test-vendor".to_string(),
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
            vec![
                Item {
                    id: "test-item".to_string(),
                    name: "Test Item".to_string(),
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
                },
            ],
        );
        
        // Encode the document
        let encoded = encode_document_as_param(&doc).unwrap();
        
        // Decode the document
        let decoded = decode_document_from_param(&encoded).unwrap();
        
        // Verify
        assert_eq!(decoded.vendor.id, "test-vendor");
        assert_eq!(decoded.vendor.name, "Test Vendor");
        assert_eq!(decoded.items.len(), 1);
        assert_eq!(decoded.items[0].name, "Test Item");
    }
}