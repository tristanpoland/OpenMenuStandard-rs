// src/types.rs
//
// Core data structures for the OpenMenuStandard

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

/// Main OmsDocument struct representing a complete OpenMenuStandard document
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OmsDocument {
    /// Version of the OpenMenuStandard specification
    pub oms_version: String,
    
    /// Metadata about the document
    pub metadata: Metadata,
    
    /// Information about the vendor
    pub vendor: Vendor,
    
    /// Menu items
    pub items: Vec<Item>,
    
    /// Optional order information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    
    /// Optional vendor-specific extensions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Extensions>,
}

/// Metadata about the OMS document
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Metadata {
    /// When the document was created
    pub created: DateTime<Utc>,
    
    /// Application or system that generated the document
    pub source: String,
    
    /// Primary language of text content (RFC 5646 language tag)
    pub locale: String,
}

/// Information about the food service provider
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Vendor {
    /// Unique identifier for the vendor
    pub id: String,
    
    /// Name of the vendor
    pub name: String,
    
    /// Type of food service
    pub r#type: String,
    
    /// Optional specific location identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    
    /// Optional specific location name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    
    /// Optional address information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    
    /// Optional contact information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    
    /// Optional business hours
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours: Option<Vec<BusinessHours>>,
    
    /// Optional cuisine types
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cuisine: Option<Vec<String>>,
    
    /// Optional available services
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<String>>,
}

/// Physical address information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Address {
    /// Street address
    pub street: String,
    
    /// City
    pub city: String,
    
    /// State or province
    pub region: String,
    
    /// Postal code
    pub postal_code: String,
    
    /// Country
    pub country: String,
}

/// Contact information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Contact {
    /// Phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    
    /// Website URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}

/// Business hours for a particular day
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BusinessHours {
    /// Day of the week
    pub day: DayOfWeek,
    
    /// Time ranges when the business is open
    pub ranges: Vec<TimeRange>,
}

/// Days of the week
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

/// Time range with open and close times
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimeRange {
    /// Opening time in 24-hour format (HH:MM)
    pub open: String,
    
    /// Closing time in 24-hour format (HH:MM)
    pub close: String,
}

/// Representation of a food or beverage item
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Item {
    /// Unique identifier for the item
    pub id: String,
    
    /// Name of the item
    pub name: String,
    
    /// Category of the item
    pub category: String,
    
    /// Optional vendor-specific identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<String>,
    
    /// Optional detailed description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Optional subcategory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subcategory: Option<String>,
    
    /// Optional URL to an image of the item
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    
    /// Optional base price before customizations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_price: Option<f64>,
    
    /// Optional currency code (ISO 4217)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    
    /// Optional nutritional information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nutrition: Option<Nutrition>,
    
    /// Optional available customizations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customizations: Option<Vec<Customization>>,
    
    /// Optional selected customizations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_customizations: Option<Vec<SelectedCustomization>>,
    
    /// Optional quantity of this item in an order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u32>,
    
    /// Optional note specific to this item
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_note: Option<String>,
    
    /// Optional calculated values based on customizations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculated: Option<CalculatedValues>,
    
    /// Optional component items for combo meals
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Item>>,
    
    /// Optional availability information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability: Option<Availability>,
    
    /// Optional popularity metrics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub popularity: Option<Popularity>,
}

/// Nutritional information about an item
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Nutrition {
    /// Serving size information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serving_size: Option<MeasurementValue>,
    
    /// Calories per serving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calories: Option<f64>,
    
    /// Protein content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protein: Option<MeasurementValue>,
    
    /// Fat content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fat: Option<NutrientWithDetails>,
    
    /// Carbohydrate content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carbohydrates: Option<NutrientWithDetails>,
    
    /// Sodium content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sodium: Option<MeasurementValue>,
    
    /// Cholesterol content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cholesterol: Option<MeasurementValue>,
    
    /// Vitamin content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vitamins: Option<Vec<VitaminMineral>>,
    
    /// Mineral content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minerals: Option<Vec<VitaminMineral>>,
    
    /// List of allergens present
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allergens: Option<Vec<String>>,
    
    /// Dietary flags for the item
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dietary_flags: Option<Vec<String>>,
    
    /// Health claims associated with the item
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_claims: Option<Vec<String>>,
    
    /// Ingredient information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingredients: Option<Vec<IngredientGroup>>,
    
    /// Nutrition standards compliance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nutrition_standards: Option<NutritionStandards>,
}

/// Measurement value with unit
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MeasurementValue {
    /// Numeric value
    pub value: f64,
    
    /// Unit of measurement
    pub unit: String,
}

/// Nutrient with detailed breakdown
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NutrientWithDetails {
    /// Numeric value
    pub value: f64,
    
    /// Unit of measurement
    pub unit: String,
    
    /// Optional detailed breakdown
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<HashMap<String, NutrientValue>>,
}

/// Nutrient value, which can be either a simple measurement or another detailed breakdown
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum NutrientValue {
    Simple(MeasurementValue),
    Detailed(NutrientWithDetails),
}

/// Vitamin or mineral nutrient information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VitaminMineral {
    /// Name of the vitamin or mineral
    pub name: String,
    
    /// Numeric value
    pub value: f64,
    
    /// Unit of measurement
    pub unit: String,
    
    /// Optional percentage of daily value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_value_percent: Option<f64>,
}

/// Ingredient grouping
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IngredientGroup {
    /// Name of the ingredient group
    pub name: String,
    
    /// List of ingredients in this group
    pub ingredients: Vec<String>,
}

/// Nutrition standards compliance information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NutritionStandards {
    /// US FDA compliance information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_fda: Option<UsFdaInfo>,
    
    /// EU regulation compliance information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_regulation: Option<EuRegulationInfo>,
}

/// US FDA nutrition information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UsFdaInfo {
    /// Serving size description
    pub serving_size_description: String,
    
    /// Daily value reference year
    pub daily_value_year: u16,
}

/// EU regulation nutrition information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EuRegulationInfo {
    /// Reference intake description
    pub reference_intake_description: String,
}

/// Customization options for an item
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Customization {
    /// Unique identifier for the customization
    pub id: String,
    
    /// Name of the customization
    pub name: String,
    
    /// Type of customization
    pub r#type: CustomizationType,
    
    /// Whether this customization is required
    #[serde(default)]
    pub required: bool,
    
    /// Default value for this customization
    pub default: CustomizationDefault,
    
    /// Minimum number of selections (for multi_select)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_selections: Option<u32>,
    
    /// Maximum number of selections (for multi_select)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_selections: Option<u32>,
    
    /// Minimum value (for quantity or range)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    
    /// Maximum value (for quantity or range)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    
    /// Step value (for quantity or range)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<f64>,
    
    /// Unit price adjustment per quantity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price_adjustment: Option<f64>,
    
    /// Unit nutrition adjustments per quantity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_nutrition_adjustments: Option<HashMap<String, NutrientValue>>,
    
    /// Available options for selection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<CustomizationOption>>,
}

/// Types of customizations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CustomizationType {
    SingleSelect,
    MultiSelect,
    Quantity,
    Boolean,
    Text,
    Range,
}

impl FromStr for CustomizationType {
    type Err = crate::OmsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "single_select" => Ok(CustomizationType::SingleSelect),
            "multi_select" => Ok(CustomizationType::MultiSelect),
            "quantity" => Ok(CustomizationType::Quantity),
            "boolean" => Ok(CustomizationType::Boolean),
            "text" => Ok(CustomizationType::Text),
            "range" => Ok(CustomizationType::Range),
            _ => Err(crate::OmsError::InvalidCustomizationType(s.to_string())),
        }
    }
}

impl fmt::Display for CustomizationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomizationType::SingleSelect => write!(f, "single_select"),
            CustomizationType::MultiSelect => write!(f, "multi_select"),
            CustomizationType::Quantity => write!(f, "quantity"),
            CustomizationType::Boolean => write!(f, "boolean"),
            CustomizationType::Text => write!(f, "text"),
            CustomizationType::Range => write!(f, "range"),
        }
    }
}

/// Default value for a customization, which varies by type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CustomizationDefault {
    String(String),
    StringArray(Vec<String>),
    Number(f64),
    Boolean(bool),
}

/// Individual option for a customization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomizationOption {
    /// Unique identifier for the option
    pub id: String,
    
    /// Name of the option
    pub name: String,
    
    /// Optional price adjustment for selecting this option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_adjustment: Option<f64>,
    
    /// Optional nutrition adjustments for selecting this option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nutrition_adjustments: Option<HashMap<String, NutrientValue>>,
    
    /// Optional allergens added by this option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allergens: Option<Vec<String>>,
    
    /// Optional dietary flags for this option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dietary_flags: Option<Vec<String>>,
}

/// Selected customization for an item
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SelectedCustomization {
    /// ID of the customization being selected
    pub customization_id: String,
    
    /// Selection value, which varies by customization type
    pub selection: CustomizationSelection,
}

/// Value of a selected customization, which varies by type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CustomizationSelection {
    String(String),
    StringArray(Vec<String>),
    Number(f64),
    Boolean(bool),
}

/// Calculated values based on customizations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalculatedValues {
    /// Calculated item price after customizations
    pub item_price: f64,
    
    /// Adjusted nutrition values after customizations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjusted_nutrition: Option<HashMap<String, f64>>,
}

/// Item availability information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Availability {
    /// Optional start date for seasonal items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    
    /// Optional end date for seasonal items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    
    /// Optional times of day when item is available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub times_of_day: Option<Vec<String>>,
    
    /// Optional days of week when item is available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_of_week: Option<Vec<String>>,
}

/// Item popularity metrics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Popularity {
    /// Optional ranking among menu items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<u32>,
    
    /// Optional descriptive tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// Order information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Order {
    /// Unique identifier for the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    
    /// Current status of the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatus>,
    
    /// When the order was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,
    
    /// Requested pickup time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup_time: Option<DateTime<Utc>>,
    
    /// Requested delivery time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_time: Option<DateTime<Utc>>,
    
    /// Type of order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<OrderType>,
    
    /// Special instructions for the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_notes: Option<String>,
    
    /// Payment information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<Payment>,
    
    /// Customer information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
    
    /// Delivery information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<Delivery>,
}

/// Order status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Draft,
    Submitted,
    Confirmed,
    InProgress,
    Ready,
    Completed,
    Cancelled,
}

/// Order type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    Pickup,
    Delivery,
    DineIn,
}

/// Payment information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Payment {
    /// Payment status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PaymentStatus>,
    
    /// Payment method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    
    /// Subtotal before tax and tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtotal: Option<f64>,
    
    /// Tax amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<f64>,
    
    /// Tip amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip: Option<f64>,
    
    /// Total amount
    pub total: f64,
    
    /// Currency code (ISO 4217)
    pub currency: String,
}

/// Payment status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PaymentStatus {
    Unpaid,
    Paid,
}

/// Customer information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Customer {
    /// Unique identifier for the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    
    /// Customer name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    /// Customer phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    
    /// Customer email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// Delivery information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Delivery {
    /// Delivery address
    pub address: Address,
    
    /// Special delivery instructions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
}

/// Extensions for vendor-specific or future extensions
pub type Extensions = HashMap<String, serde_json::Value>;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_customization_type_display() {
        assert_eq!(CustomizationType::SingleSelect.to_string(), "single_select");
        assert_eq!(CustomizationType::MultiSelect.to_string(), "multi_select");
        assert_eq!(CustomizationType::Quantity.to_string(), "quantity");
        assert_eq!(CustomizationType::Boolean.to_string(), "boolean");
        assert_eq!(CustomizationType::Text.to_string(), "text");
        assert_eq!(CustomizationType::Range.to_string(), "range");
    }
    
    #[test]
    fn test_customization_type_from_str() {
        assert_eq!(
            CustomizationType::from_str("single_select").unwrap(),
            CustomizationType::SingleSelect
        );
        assert_eq!(
            CustomizationType::from_str("multi_select").unwrap(),
            CustomizationType::MultiSelect
        );
        assert!(CustomizationType::from_str("invalid").is_err());
    }
}