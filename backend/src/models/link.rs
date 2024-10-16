use super::node_edge_point::NodeEdgePoint;
use crate::Error; // Import custom error handling type `Error` from the crate // Import the `NodeEdgePoint` struct from a sibling module

// Import necessary traits for hashing
use std::hash::{DefaultHasher, Hash, Hasher};

// Import date and time utilities from the `chrono` crate
use chrono::{DateTime, Local};

// Import serialization and deserialization traits from `serde`
use serde::{Deserialize, Serialize};

// Import JSON utilities for working with `serde_json`
// `Value` is used for dynamic JSON parsing, `to_string` is used for serialization
use serde_json::{to_string, Value};

// Import UUID handling utilities from the `uuid` crate
use uuid::Uuid;

// Define the `Link` struct with relevant fields, and make it serializable, deserializable, and comparable
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Link {
    pub host: String,
    #[serde(rename(serialize = "node-edge-point", deserialize = "node-edge-point"))]
    // Rename field for (de)serialization
    pub node_edge_points: Vec<NodeEdgePoint>, // A vector of node-edge points
    pub uuid: Uuid,            // A UUID for identifying the link
    pub hash: u64,             // A hash for identifying changes in the link object
    pub date: DateTime<Local>, // Timestamp for when the link was created or last modified
}

impl Link {
    /// Creates a Link instance from a JSON `Value` and host
    ///
    /// # Arguments
    /// - `value`: A reference to the JSON `Value` to deserialize from
    /// - `host`: A reference to the host `static str`
    ///
    /// # Returns
    /// - `Ok(Device)`: If the deserialization is successful
    /// - `Err(Error)`: If required fields are missing or invalid
    pub fn from_value(value: &Value, host: &'static str) -> Result<Self, Error> {
        let host = host.to_string();

        // Parse the UUID from the input `Value`
        let uuid: Uuid = Uuid::parse_str(
            &value
                .get("uuid") // Try to get the `uuid` field
                .and_then(Value::as_str) // Ensure it's a string
                .unwrap_or_default(), // Default to an empty string if not found
        )
        .map_err(|_| Error::from("Not found link uuid"))?; // Return an error if parsing fails

        // Get the array of node-edge points from the JSON `Value`
        let node_edge_points_array: &Vec<Value> = value
            .get("node-edge-point") // Try to get `node-edge-point` field
            .and_then(Value::as_array) // Ensure it's an array
            .ok_or_else(|| Error::from("Not found node edge points list"))?; // Return error if not found

        // Initialize an empty vector to store parsed node-edge points
        let mut node_edge_points: Vec<NodeEdgePoint> = vec![];

        // Iterate over the array of node-edge points and try to parse each into a `NodeEdgePoint`
        for node_edge_point in node_edge_points_array {
            match NodeEdgePoint::from_value(node_edge_point) {
                // Clone each `Value` and parse it
                Ok(ok_node_edge_point) => node_edge_points.push(ok_node_edge_point), // Add to the list if successful
                Err(err) => {
                    // If an error occurs during parsing, return it
                    return Err(err);
                }
            }
        }

        // Create a new hasher instance
        let mut hasher = DefaultHasher::new();
        // Hash the string representation of the entire `value` (JSON structure)
        to_string(&value).unwrap().hash(&mut hasher);
        // Get the current timestamp using `chrono::Local`
        let now = Local::now();

        // Return a new `Link` object populated with the parsed data
        Ok(Link {
            host: host,
            node_edge_points: node_edge_points, // Parsed node-edge points
            uuid: uuid,                         // Parsed UUID
            hash: hasher.finish(),              // The calculated hash value
            date: now,                          // The current timestamp
        })
    }
}
