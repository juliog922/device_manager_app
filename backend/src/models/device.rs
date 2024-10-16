use crate::Error; // Import custom error handling type `Error` from the crate

// Import necessary traits for serialization and deserialization
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a Device with host, port, and authentication type
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Device {
    pub host: String,      // Host name or IP address of the device
    pub port: Option<i64>, // Optional port number
    pub auth: Auth,        // Authentication method (enum)
}

impl Device {
    /// Creates a Device instance from a JSON `Value`
    ///
    /// # Arguments
    /// - `value`: A reference to the JSON `Value` to deserialize from
    ///
    /// # Returns
    /// - `Ok(Device)`: If the deserialization is successful
    /// - `Err(Error)`: If required fields are missing or invalid
    pub fn from_value(value: &Value) -> Result<Self, Error> {
        // Extract the host field from the JSON
        let host_value = value
            .get("host")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::from("Host not found"))?;

        // Extract the optional port field from the JSON
        let port_value = value.get("port").and_then(Value::as_i64);

        // Extract and deserialize the authentication field (which is of enum type Auth)
        let auth_value = Auth::from_value(
            value
                .get("auth")
                .ok_or_else(|| Error::from("Auth body not found"))?,
        )?;

        // Return a Device instance
        Ok(Device {
            host: host_value.to_string(),
            port: port_value,
            auth: auth_value,
        })
    }
}

/// Enum representing the different authentication methods
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Auth {
    BasicAuth(BasicAuth), // Basic Authentication
    Oauth2(Oauth2),       // OAuth2 Authentication
    Custom(CustomAuth),   // Custom Authentication
}

impl Auth {
    /// Creates an Auth enum from a JSON `Value`
    ///
    /// # Arguments
    /// - `value`: A reference to the JSON `Value` to deserialize from
    ///
    /// # Returns
    /// - `Ok(Auth)`: If the deserialization is successful and determines the correct enum variant
    /// - `Err(Error)`: If the fields do not match any known authentication type
    pub fn from_value(value: &Value) -> Result<Auth, Error> {
        // Extract the object (hash map) from the JSON value to inspect the fields
        let value_object = value
            .as_object()
            .ok_or_else(|| Error::from("Auth body not valid"))?;

        // Determine the correct Auth variant based on the fields present in the object
        if value_object.contains_key("grant_type") {
            // OAuth2 authentication
            let auth = Oauth2::from_value(&value)?;
            Ok(Auth::Oauth2(auth))
        } else if value_object.contains_key("auth_body") {
            // Custom authentication
            let auth = CustomAuth::from_value(&value)?;
            Ok(Auth::Custom(auth))
        } else if value_object.contains_key("username") && value_object.contains_key("password") {
            // Basic authentication
            let auth = BasicAuth::from_value(&value)?;
            Ok(Auth::BasicAuth(auth))
        } else {
            // If no recognizable fields, return an error
            Err(Error::from("Not recognizable authentication type"))
        }
    }
}

/// Represents Basic Authentication with username and password
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BasicAuth {
    pub username: String, // Username for authentication
    pub password: String, // Password for authentication
}

impl BasicAuth {
    /// Creates a BasicAuth instance from a JSON `Value`
    ///
    /// # Arguments
    /// - `value`: A reference to the JSON `Value` to deserialize from
    ///
    /// # Returns
    /// - `Ok(BasicAuth)`: If deserialization is successful
    /// - `Err(Error)`: If required fields are missing
    pub fn from_value(value: &Value) -> Result<BasicAuth, Error> {
        let username_value = value
            .get("username")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::from("Username for Basic authentication not found"))?;
        let password_value = value
            .get("password")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::from("Password for Basic authentication not found"))?;

        Ok(BasicAuth {
            username: username_value.to_string(),
            password: password_value.to_string(),
        })
    }
}

/// Represents OAuth2 Authentication with additional fields for grant type and authentication URL
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Oauth2 {
    pub username: String,   // Username for OAuth2 authentication
    pub password: String,   // Password for OAuth2 authentication
    pub grant_type: String, // Grant type for OAuth2 (e.g., client_credentials, password)
    pub auth_url: String,   // URL to request OAuth2 token
}

impl Oauth2 {
    /// Creates an Oauth2 instance from a JSON `Value`
    ///
    /// # Arguments
    /// - `value`: A reference to the JSON `Value` to deserialize from
    ///
    /// # Returns
    /// - `Ok(Oauth2)`: If deserialization is successful
    /// - `Err(Error)`: If required fields are missing
    pub fn from_value(value: &Value) -> Result<Oauth2, Error> {
        let username_value = value
            .get("username")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::from("Username for OAuth2 authentication not found"))?;
        let password_value = value
            .get("password")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::from("Password for OAuth2 authentication not found"))?;
        let grant_type_value = value
            .get("grant_type")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::from("Grant type for OAuth2 authentication not found"))?;
        let auth_url_value = value
            .get("auth_url")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::from("Authentication URL for OAuth2 authentication not found"))?;

        Ok(Oauth2 {
            username: username_value.to_string(),
            password: password_value.to_string(),
            grant_type: grant_type_value.to_string(),
            auth_url: auth_url_value.to_string(),
        })
    }
}

/// Represents Custom Authentication with an arbitrary body and authentication URL
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CustomAuth {
    pub auth_body: Value, // A JSON object containing custom authentication data
    pub auth_url: String, // URL for custom authentication
}

impl CustomAuth {
    /// Creates a CustomAuth instance from a JSON `Value`
    ///
    /// # Arguments
    /// - `value`: A reference to the JSON `Value` to deserialize from
    ///
    /// # Returns
    /// - `Ok(CustomAuth)`: If deserialization is successful
    /// - `Err(Error)`: If required fields are missing
    pub fn from_value(value: &Value) -> Result<CustomAuth, Error> {
        let auth_body_value = value.get("auth_body").ok_or_else(|| {
            Error::from("Authentication body for Custom authentication not found")
        })?;
        let auth_url_value = value
            .get("auth_url")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::from("Authentication URL for Custom authentication not found"))?;

        Ok(CustomAuth {
            auth_body: auth_body_value.clone(),
            auth_url: auth_url_value.to_string(),
        })
    }
}
