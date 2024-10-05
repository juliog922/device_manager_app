# TAPI Services Schema in a Device

This `README.md` describes an application that exposes the schema of each service within a TAPI-compatible device. The application presents a structured view of the services, starting from nodes, physical cards (inventory_id), and endpoints that connect via client/parent connections, link connections, or lower connections.

## Information Organization:
- **Node Level**: The application shows information for each node within the device, identified by their `node_uuid`. Within each node, physical cards are listed, identified by their `inventory_id`. This `inventory_id` correlates with legacy hardware identifiers.
  - Example of `inventory_id` format: `/ne=Barcelona/r=1/sh=3/sl=7/s_sl=2/p=2`
  
- **Physical Card Level (inventory_id)**: For each physical card, the application shows a list of endpoints.
  - Example of how an endpoint relates to its physical card:
  
- **Endpoint Level**: Endpoints are represented by the `connection-end-point` object and can be obtained through the `cep-list` attribute of an `owned-node-edge-point` (NEP).
  - Relevant parameters for the endpoint:
    - `connection_end_point_uuid`: Unique identifier of the endpoint.
    - `layer_protocol_qualifier`: Indicates the type of digital signal used by the endpoint. Obtained from the `layer-protocol-qualifier` of the associated NEP.
    - `node_edge_point_uuid`: Identifies the Node Edge Point (NEP) to which the endpoint is associated. Obtained from the `uuid` of the associated NEP.
    - `service_interface_point_uuid`: If the NEP supports a SIP, the `uuid` of the SIP should be in the `mapped-service-interface-point` list of the NEP.

## Connections:
The application describes how endpoints connect to each other through different types of connections:
- **Client/Parent Connection**: This relationship is established between a Client Node Edge Point (NEP) and a Parent NEP.
  - `client_node_edge_point`: List of references to NEPs that are clients of this endpoint.
  - `parent_node_edge_point`: Reference to the NEP that is the parent of this endpoint.
  
- **Link Connection**: Associated with a TAPI link (`tapi-topology:link`), used to model the effective adjacency between two or more node instances in a topology. Accessible through the `supported-client-link` attribute in a connection object. This attribute is relevant when working with explicit links in the topology, such as photonic media links (`PHOTONIC_MEDIA`) that may be generated when provisioning connectivity services.

- **Lower Connection**: A Top Connection must include a reference to all lower connections (`lower-connection`) that support it in the same network layer and qualifier. These lower connections represent the realization of the service in lower layers. For example, a DSR connection can have lower connections in the OTN and Photonic Media layers.

## Service Schema Example:
The following code snippet shows an example of the data structure that the application could expose. Note that this example is a simplification, and the real structure may vary depending on the device configuration.

```json
"connectivity_services": [
  {
    "nodes": [
      {
        "node_id": "node-1",
        "inventory_id": "/ne=Barcelona/r=1/sh=1", 
        "end_points": [
          {
            "connection_end_point_uuid": "endpoint-1-1",
            "layer_protocol_qualifier": "tapi-dsr:DIGITAL_SIGNAL_TYPE_10_GigE_LAN",
            "node_edge_point_uuid": "nep-1-1",
            "service_interface_point_uuid": "sip-1-1",
            "client_node_edge_point_uuid": "nep-2-1", 
            "parent_node_edge_point": "nep-1-2" 
          },
          {
            "connection_end_point_uuid": "endpoint-1-2",
            // ... other endpoint parameters ...
          }
        ],
        "lower_connections": [
          {
            "connection_uuid": "lower-connection-1-1",
            // ... other connection parameters ...
          }
        ]
      },
      {
        "node_id": "node-2",
        "inventory_id": "/ne=Madrid/r=2/sh=2",
        "end_points": [
          {
            "connection_end_point_uuid": "endpoint-2-1",
            // ... other endpoint parameters ...
          }
        ],
        "lower_connections": [
          {
            "connection_uuid": "lower-connection-2-1",
            // ... other connection parameters ...
          }
        ]
      }
    ]
  }
]
```

## Conclusion:
This application provides a useful tool for understanding the structure of services within a TAPI-compatible device. By exposing information about nodes, physical cards, endpoints, and the connections between them, the application facilitates network service management, diagnostics, and configuration.
