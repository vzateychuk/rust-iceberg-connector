use cloudevents::event::Event;
use serde_json::Value;
use std::error::Error;

// use cloudevents_sdk::event::Event;
/// Processes a CloudEvent given as a JSON string.
pub async fn handle_event(json: &str) -> Result<(), Box<dyn Error>> {
    // Parse raw JSON into serde_json::Value
    let json_value: Value = serde_json::from_str(json)?;

    // Convert into CloudEvent using cloudevents-sdk
    let event = Event::from_json_value(json_value)?;

    // Validate basic required fields
    if event.id().is_empty() || event.ty().is_empty() || event.source().is_empty() {
        return Err("Invalid CloudEvent: missing required fields".into());
    }

    println!("Received CloudEvent: id={}, type={}, source={}",
        event.id(), event.ty(), event.source());

    // Forward to Iceberg writer (stub)
    crate::iceberg_writer::write_event_to_iceberg(&event).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_valid_cloudevent() {
        let json = r#"{
            "specversion": "1.0",
            "id": "abc-123",
            "source": "/test",
            "type": "com.example.event",
            "data": {"message": "Hello World"}
        }"#;

        let result = handle_event(json).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_invalid_cloudevent_missing_fields() {
        let json = r#"{
            "specversion": "1.0",
            "id": "",
            "source": "",
            "type": "",
            "data": {}
        }"#;

        let result = handle_event(json).await;
        assert!(result.is_err());
    }
}
// This file handles the ingestion of CloudEvents, parsing them from JSON,
// validating required fields, and forwarding them to the Iceberg writer stub.
// It includes tests for both valid and invalid events.
// The Iceberg writer stub is expected to be implemented in the iceberg_writer module.
// The code uses cloudevents-sdk for event handling and serde_json for JSON parsing.
// The tests use Tokio for async execution and validate the event handling logic.
// The main function is not included here as this is a library module.
