// Function to match the scheduled job function with its corresponding handler.
// In case you want to add a new one, just add a new clause to the match with 
// the job name and the corresponding function.

// The metadata is a serde_json::Value, please visit: https://docs.rs/serde_json/latest/serde_json/value/enum.Value.html
// to refer on how to get values from there.
// Example of accessing an integer id in the metadata:
//    metadata["id"].as_i64().unwrap();

pub async fn handle_job(name: &String, metadata: &serde_json::Value) -> anyhow::Result<()> {
    match name {
      _ => default(&name, &metadata)
    }
}

fn default(name: &String, metadata: &serde_json::Value) -> anyhow::Result<()> {
  println!("handle_job fell into default case: (name={:?}, metadata={:?})", name, metadata);
  tracing::trace!("handle_job fell into default case: (name={:?}, metadata={:?})", name, metadata);

  Ok(())
}
