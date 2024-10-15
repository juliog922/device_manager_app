use backend::models::{
    // Import necessary model components
    link::Link,
    node_edge_point::NodeEdgePoint,
};
use backend::Error; // Import the custom error type from the backend module
use chrono::Local; // For handling date and time
use serde_json::{
    from_str,
    to_string,
    // Importing JSON serialization/deserialization utilities
    Value,
};
use std::hash::{
    // Import hashing traits
    DefaultHasher,
    Hash,
    Hasher,
};
use uuid::Uuid; // For handling UUIDs (universally unique identifiers)

/// # Test: `test_raw_link`
///
/// This test verifies that a raw JSON string containing a `Link` object
/// can be correctly parsed into a `Link` struct and that two identical
/// `Link` structs are considered equal. It also tests if the
/// `node_edge_points` field and other fields are populated correctly.
#[test]
fn test_raw_link() {
    // Example of raw JSON data representing a `Link` object
    let raw_link_data = r#"
        {
            "administrative-state": "UNLOCKED",
            "direction": "BIDIRECTIONAL",
            "layer-protocol-name": [
                "ETH"
            ],
            "lifecycle-state": "INSTALLED",
            "name": [
                {
                    "value": "",
                    "value-name": "LINK_NAME"
                }
            ],
            "node-edge-point": [
                {
                    "node-edge-point-uuid": "65a39427-3055-3ba4-9e15-0ebed4974577",
                    "node-uuid": "62d11f13-db6c-3398-8a83-5fac0b2b7476",
                    "topology-uuid": "4e537278-79f8-39ad-804b-f0b553cb2ffb"
                },
                {
                    "node-edge-point-uuid": "63366151-aeb4-3dfd-af66-d471b353aa1c",
                    "node-uuid": "7b0c973a-996a-3409-ad2f-d173354bfdb7",
                    "topology-uuid": "4e537278-79f8-39ad-804b-f0b553cb2ffb"
                }
            ],
            "operational-state": "ENABLED",
            "resilience-type": {
                "protection-type": "NO_PROTECTON",
                "restoration-policy": "NA"
            },
            "tapi-ciena-link-extensions:layer-protocol-qualifier": "tapi-ciena-protocol-extensions:ETHERNET",
            "tapi-ciena-link-extensions:signal-content-type": "IP",
            "uuid": "14219539-208b-35f5-b7cf-35a58e083490"
        }"#;

    // Deserialize raw JSON data into a `Value` type and unwrap safely
    let raw_link_data_value: Value = from_str(&raw_link_data).unwrap_or_default();
    // Attempt to create a `Link` object from the `Value`
    let raw_link_object = Link::from_value(raw_link_data_value).unwrap();

    // Manually create a `Link` object with the same data
    let second_link_object: Link = Link {
        node_edge_points: vec![
            NodeEdgePoint {
                node_edge_point_uuid: Uuid::parse_str("65a39427-3055-3ba4-9e15-0ebed4974577")
                    .unwrap_or_default(),
                node_uuid: Uuid::parse_str("62d11f13-db6c-3398-8a83-5fac0b2b7476")
                    .unwrap_or_default(),
            },
            NodeEdgePoint {
                node_edge_point_uuid: Uuid::parse_str("63366151-aeb4-3dfd-af66-d471b353aa1c")
                    .unwrap_or_default(),
                node_uuid: Uuid::parse_str("7b0c973a-996a-3409-ad2f-d173354bfdb7")
                    .unwrap_or_default(),
            },
        ],
        uuid: Uuid::parse_str("14219539-208b-35f5-b7cf-35a58e083490").unwrap_or_default(),
        hash: raw_link_object.hash,
        date: raw_link_object.date,
    };

    // Assert that both manually created and parsed `Link` objects are equal
    assert_eq!(raw_link_object, second_link_object);
}

/// # Test: `test_raw_link_error`
///
/// This test checks if the correct errors are returned when expected.
/// It simulates incorrect or incomplete data and verifies that the appropriate
/// error messages are triggered in response.
#[test]
fn test_raw_link_error() {
    let raw_link_data = r#"
        {
            "administrative-state": "UNLOCKED",
            "direction": "BIDIRECTIONAL",
            "layer-protocol-name": [
                "ETH"
            ],
            "lifecycle-state": "INSTALLED",
            "name": [
                {
                    "value": "",
                    "value-name": "LINK_NAME"
                }
            ],
            "node-edge-point": [
                {
                    "node-edge-point-uuid": "65a39427-3055-3ba4-9e15-0ebed4974577",
                    "node-uuid": "62d11f13-db6c-3398-8a83-5fac0b2b7476",
                    "topology-uuid": "4e537278-79f8-39ad-804b-f0b553cb2ffb"
                },
                {
                    "node-edge-point-uuid": "63366151-aeb4-3dfd-af66-d471b353aa1c",
                    "topology-uuid": "4e537278-79f8-39ad-804b-f0b553cb2ffb"
                }
            ],
            "operational-state": "ENABLED",
            "resilience-type": {
                "protection-type": "NO_PROTECTON",
                "restoration-policy": "NA"
            },
            "tapi-ciena-link-extensions:layer-protocol-qualifier": "tapi-ciena-protocol-extensions:ETHERNET",
            "tapi-ciena-link-extensions:signal-content-type": "IP",
            "uuid": "14219539-208b-35f5-b7cf-35a58e083490"
        }"#;

    let raw_link_data_value: Value = from_str(&raw_link_data).unwrap_or_default();

    // Check for a custom error when certain required fields are missing
    match Link::from_value(raw_link_data_value) {
        Err(e) => {
            match e {
                Error::Custom(msg) => {
                    assert_eq!(msg, "Not found node uuid".to_string());
                }
                //_ => panic!("Expected an Error::Custom, but got other kind of error")
            }
        }
        Ok(_) => panic!("Expected an error, but got Ok"),
    }

    // Repeat similar tests for other potential errors
    // [...]
    let raw_link_data = r#"
        {
            "administrative-state": "UNLOCKED",
            "direction": "BIDIRECTIONAL",
            "layer-protocol-name": [
                "ETH"
            ],
            "lifecycle-state": "INSTALLED",
            "name": [
                {
                    "value": "",
                    "value-name": "LINK_NAME"
                }
            ],
            "node-edge-point": [
                {
                    "node-edge-point-uuid": "65a39427-3055-3ba4-9e15-0ebed4974577",
                    "node-uuid": "62d11f13-db6c-3398-8a83-5fac0b2b7476",
                    "topology-uuid": "4e537278-79f8-39ad-804b-f0b553cb2ffb"
                },
                {
                    "node-uuid": "62d11f13-db6c-3398-8a83-5fac0b2b7476",
                    "topology-uuid": "4e537278-79f8-39ad-804b-f0b553cb2ffb"
                }
            ],
            "operational-state": "ENABLED",
            "resilience-type": {
                "protection-type": "NO_PROTECTON",
                "restoration-policy": "NA"
            },
            "tapi-ciena-link-extensions:layer-protocol-qualifier": "tapi-ciena-protocol-extensions:ETHERNET",
            "tapi-ciena-link-extensions:signal-content-type": "IP",
            "uuid": "14219539-208b-35f5-b7cf-35a58e083490"
        }"#;

    let raw_link_data_value: Value = from_str(&raw_link_data).unwrap_or_default();
    match Link::from_value(raw_link_data_value) {
        Err(e) => {
            match e {
                Error::Custom(msg) => {
                    assert_eq!(msg, "Not found node edge point uuid".to_string());
                }
                //_ => panic!("Expected an Error::Custom, but got other kind of error")
            }
        }
        Ok(_) => panic!("Expected an error, but got Ok"),
    }

    let raw_link_data = r#"
        {
            "administrative-state": "UNLOCKED",
            "direction": "BIDIRECTIONAL",
            "layer-protocol-name": [
                "ETH"
            ],
            "lifecycle-state": "INSTALLED",
            "name": [
                {
                    "value": "",
                    "value-name": "LINK_NAME"
                }
            ],
            "node-edge-point": [
                {
                    "node-edge-point-uuid": "65a39427-3055-3ba4-9e15-0ebed4974577",
                    "node-uuid": "62d11f13-db6c-3398-8a83-5fac0b2b7476",
                    "topology-uuid": "4e537278-79f8-39ad-804b-f0b553cb2ffb"
                },
                {
                    "node-edge-point-uuid": "65a39427-3055-3ba4-9e15-0ebed4974577",
                    "node-uuid": "62d11f13-db6c-3398-8a83-5fac0b2b7476",
                    "topology-uuid": "4e537278-79f8-39ad-804b-f0b553cb2ffb"
                }
            ],
            "operational-state": "ENABLED",
            "resilience-type": {
                "protection-type": "NO_PROTECTON",
                "restoration-policy": "NA"
            },
            "tapi-ciena-link-extensions:layer-protocol-qualifier": "tapi-ciena-protocol-extensions:ETHERNET",
            "tapi-ciena-link-extensions:signal-content-type": "IP"
        }"#;

    let raw_link_data_value: Value = from_str(&raw_link_data).unwrap_or_default();
    match Link::from_value(raw_link_data_value) {
        Err(e) => {
            match e {
                Error::Custom(msg) => {
                    assert_eq!(msg, "Not found link uuid".to_string());
                }
                //_ => panic!("Expected an Error::Custom, but got other kind of error")
            }
        }
        Ok(_) => panic!("Expected an error, but got Ok"),
    }

    let raw_link_data = r#"
        {
            "administrative-state": "UNLOCKED",
            "direction": "BIDIRECTIONAL",
            "layer-protocol-name": [
                "ETH"
            ],
            "lifecycle-state": "INSTALLED",
            "name": [
                {
                    "value": "",
                    "value-name": "LINK_NAME"
                }
            ],
            "node-edge-point": {
                "node-edge-point-uuid": "65a39427-3055-3ba4-9e15-0ebed4974577",
                "node-uuid": "62d11f13-db6c-3398-8a83-5fac0b2b7476",
                "topology-uuid": "4e537278-79f8-39ad-804b-f0b553cb2ffb"
            },
            "operational-state": "ENABLED",
            "resilience-type": {
                "protection-type": "NO_PROTECTON",
                "restoration-policy": "NA"
            },
            "tapi-ciena-link-extensions:layer-protocol-qualifier": "tapi-ciena-protocol-extensions:ETHERNET",
            "tapi-ciena-link-extensions:signal-content-type": "IP",
            "uuid": "14219539-208b-35f5-b7cf-35a58e083490"
        }"#;

    let raw_link_data_value: Value = from_str(&raw_link_data).unwrap_or_default();
    match Link::from_value(raw_link_data_value) {
        Err(e) => {
            match e {
                Error::Custom(msg) => {
                    assert_eq!(msg, "Not found node edge points list".to_string());
                }
                //_ => panic!("Expected an Error::Custom, but got other kind of error")
            }
        }
        Ok(_) => panic!("Expected an error, but got Ok"),
    }
}

/// # Test: `test_controlled_link`
///
/// This test creates a controlled `Link` object and ensures that its JSON serialization
/// and deserialization work as expected. The hash and timestamp are checked for correctness.
#[test]
fn test_controlled_link() {
    let link_data = r#"
        {
            "node-edge-point": [
                {
                    "node-edge-point-uuid": "65a39427-3055-3ba4-9e15-0ebed4974577",
                    "node-uuid": "62d11f13-db6c-3398-8a83-5fac0b2b7476"
                },
                {
                    "node-edge-point-uuid": "63366151-aeb4-3dfd-af66-d471b353aa1c",
                    "node-uuid": "7b0c973a-996a-3409-ad2f-d173354bfdb7"
                }
            ],
            "uuid": "14219539-208b-35f5-b7cf-35a58e083490"
        }"#;

    // Hashing and timestamp generation
    let mut hasher = DefaultHasher::new();
    String::from(link_data).hash(&mut hasher);
    let now = Local::now();

    // Create a `Link` object
    let link_object: Link = Link {
        node_edge_points: vec![
            NodeEdgePoint {
                node_edge_point_uuid: Uuid::parse_str("65a39427-3055-3ba4-9e15-0ebed4974577")
                    .unwrap_or_default(),
                node_uuid: Uuid::parse_str("62d11f13-db6c-3398-8a83-5fac0b2b7476")
                    .unwrap_or_default(),
            },
            NodeEdgePoint {
                node_edge_point_uuid: Uuid::parse_str("63366151-aeb4-3dfd-af66-d471b353aa1c")
                    .unwrap_or_default(),
                node_uuid: Uuid::parse_str("7b0c973a-996a-3409-ad2f-d173354bfdb7")
                    .unwrap_or_default(),
            },
        ],
        uuid: Uuid::parse_str("14219539-208b-35f5-b7cf-35a58e083490").unwrap_or_default(),
        hash: hasher.finish(),
        date: now,
    };

    // Format expected JSON output
    let link_data_formated = format!(
        r#"
    {{
        "node-edge-point": [
            {{
                "node-edge-point-uuid": "65a39427-3055-3ba4-9e15-0ebed4974577",
                "node-uuid": "62d11f13-db6c-3398-8a83-5fac0b2b7476"
            }},
            {{
                "node-edge-point-uuid": "63366151-aeb4-3dfd-af66-d471b353aa1c",
                "node-uuid": "7b0c973a-996a-3409-ad2f-d173354bfdb7"
            }}
        ],
        "uuid": "14219539-208b-35f5-b7cf-35a58e083490",
        "hash":{},
        "date":"{}"
    }}"#,
        hasher.finish(),
        now.to_rfc3339()
    );

    // Assert that serialization to JSON is successful
    assert_eq!(
        to_string(&link_object).unwrap(),
        link_data_formated.replace("\n", "").replace(" ", "")
    );
    // Assert the reverse process: deserialization works correctl
    assert_eq!(from_str::<Link>(&link_data_formated).unwrap(), link_object);
}
