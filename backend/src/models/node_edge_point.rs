use crate::Error; // Import custom error handling type `Error` from the crate

// Import necessary traits for serialization and deserialization
use serde::{Deserialize, Serialize};

// Import JSON utilities for dynamic JSON value parsing
use serde_json::Value;

// Import UUID handling utilities from the `uuid` crate
use uuid::Uuid;

// Define the `NodeEdgePoint` struct with relevant fields, and make it serializable, deserializable, and comparable
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NodeEdgePoint {
    #[serde(rename(
        serialize = "node-edge-point-uuid",
        deserialize = "node-edge-point-uuid"
    ))] // Rename field for (de)serialization
    pub node_edge_point_uuid: Uuid, // UUID for the node edge point
    #[serde(rename(serialize = "node-uuid", deserialize = "node-uuid"))]
    // Rename field for (de)serialization
    pub node_uuid: Uuid, // UUID for the node
}

impl NodeEdgePoint {
    /// Create a `NodeEdgePoint` object from a dynamic `Value` (parsed JSON)
    /// Returns `Ok(NodeEdgePoint)` if successful, or an `Err(Error)` if there's an issue
    pub fn from_value(value: &Value) -> Result<Self, Error> {
        // Parse the node edge point UUID from the input `Value`
        let node_edge_point_uuid: Uuid = Uuid::parse_str(
            &value
                .get("node-edge-point-uuid") // Try to get the `node-edge-point-uuid` field
                .and_then(Value::as_str) // Ensure it's a string
                .unwrap_or_default(), // Default to an empty string if not found
        )
        .map_err(|_| Error::from("Not found node edge point uuid"))?; // Return an error if parsing fails

        // Parse the node UUID from the input `Value`
        let node_uuid: Uuid = Uuid::parse_str(
            &value
                .get("node-uuid") // Try to get the `node-uuid` field
                .and_then(Value::as_str) // Ensure it's a string
                .unwrap_or_default(), // Default to an empty string if not found
        )
        .map_err(|_| Error::from("Not found node uuid"))?; // Return an error if parsing fails

        // Return a new `NodeEdgePoint` object populated with the parsed data
        Ok(NodeEdgePoint {
            node_edge_point_uuid: node_edge_point_uuid, // Parsed node edge point UUID
            node_uuid: node_uuid,                       // Parsed node UUID
        })
    }
}
