use log::{debug, error};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use url::Url;

#[derive(Debug, Clone)]
pub struct SchemaId {
  pub full_id: Url,
  pub relative_id: String,
}

impl SchemaId {
  /// This function will take your JSON value
  /// and extract the `$id`, converting it to a `SchemaId` value.
  ///
  /// # Examples
  ///
  /// Simple happy path example.
  ///
  /// ```rust
  /// # use bundle_schema::bundler::SchemaId;
  /// let json_str = serde_json::json!({"$id":"https://foo.com/somelocation/schema.json"});
  /// let result = SchemaId::from_json_value(&json_str);
  /// assert!(result.is_some(), "Should have `Some` value");
  /// assert_eq!(
  ///   result.as_ref().unwrap().full_id.as_str(),
  ///   "https://foo.com/somelocation/schema.json",
  ///   "Verify the full_id"
  /// );
  /// assert_eq!(
  ///   result.as_ref().unwrap().relative_id,
  ///   "somelocation/schema.json",
  ///   "Verify the relative_id"
  /// );
  /// ```
  ///
  /// Simple fail case example.
  ///
  /// ```rust
  /// # use bundle_schema::util::bundler::SchemaId;
  /// let json_str = serde_json::json!({"url":"https://foo.com/somelocation/schema.json"});
  /// let result = SchemaId::from_json_value(&json_str);
  /// assert!(result.is_none(), "Should have `None` value");
  /// ```
  pub fn from_json_value(val: &JsonValue) -> Option<Self> {
    let id_val = match val.get("$id") {
      Some(v) => v,
      None => {
        debug!("No $id value defined: {val:#?}");
        return None;
      }
    };

    let id_val_str = match id_val.as_str() {
      Some(s) => s,
      None => {
        error!("Unable to parse the $id value «{id_val:#?}» as a string.");
        return None;
      }
    };

    let id_url = match Url::parse(id_val_str) {
      Err(e) => {
        error!("Unable to parse $id value «{id_val_str}» as URL: {e:#?}");
        return None;
      }
      Ok(u) => u,
    };

    let relative_path = String::from(id_url.path().strip_prefix('/').unwrap_or(id_url.path()));

    Some(Self {
      full_id: id_url,
      relative_id: relative_path,
    })
  }
}

#[derive(Debug, Clone)]
pub struct SchemaMapItem {
  pub id: SchemaId,
  pub node: JsonValue,
}

#[derive(Debug)]
pub struct SchemaMap {
  pub registry: HashMap<String, SchemaMapItem>,
}

// Fine, clippy, I'll implement Default for SchemaMap.
impl Default for SchemaMap {
  fn default() -> Self {
    SchemaMap::new()
  }
}
impl SchemaMap {
  pub fn new() -> Self {
    SchemaMap {
      registry: HashMap::new(),
    }
  }

  /// Register a schema with the bundler
  ///
  /// # Examples
  ///
  /// ```rust
  /// # use bundle_schema::bundler::SchemaMap;
  /// let schema = serde_json::json!({
  ///   "$id":"https://foo.com/somelocation/schema.json",
  ///   "description":"I'm just a schema"
  /// });
  /// let mut registry = SchemaMap::new();
  /// registry.register_schema(schema);
  /// assert_eq!(registry.registry.len(), 1, "Verify we've got an item.");
  /// ```
  pub fn register_schema(&mut self, schema: JsonValue) {
    let id = SchemaId::from_json_value(&schema);

    if let Some(the_id) = id {
      debug!("Using ID {the_id:#?}");

      self.registry.insert(
        the_id.relative_id.clone(),
        SchemaMapItem {
          id: the_id,
          node: schema,
        },
      );
    } else {
      error!("Unable to register a schema without `$id` property.");
    }
  }

  /// Get an item from the registry by its relative ID
  ///
  /// # Examples
  ///
  /// Simple happy path
  ///
  /// ```rust
  /// # use bundle_schema::bundler::SchemaMap;
  /// # let schema = serde_json::json!({
  /// #   "$id":"https://foo.com/somelocation/schema.json",
  /// #   "description":"I'm just a schema"
  /// # });
  /// # let mut registry = SchemaMap::new();
  /// # registry.register_schema(schema.clone());
  /// let item = registry.get("somelocation/schema.json".to_owned());
  /// assert_eq!(item.unwrap(), &schema, "We got back the schema we registered!");
  /// ```
  ///
  /// Simple record-not-found case
  ///
  /// ```rust
  /// # use bundle_schema::bundler::SchemaMap;
  /// # let schema = serde_json::json!({
  /// #   "$id":"https://foo.com/somelocation/schema.json",
  /// #   "description":"I'm just a schema"
  /// # });
  /// # let mut registry = SchemaMap::new();
  /// # registry.register_schema(schema.clone());
  /// let item = registry.get("somelocation/missing-value.json".to_owned());
  /// assert!(item.is_none(), "Expected None");
  /// ```
  pub fn get(&self, which: String) -> Option<&JsonValue> {
    if let Some(item) = self.registry.get(&which) {
      return Some(&item.node);
    }

    None
  }
}

#[cfg(test)]
mod tests {
  // use super::SchemaId;
  // use crate::logging;
  // use log::debug;

  // #[test]
  // fn test_debug() {
  // logging::init_logging(true);
  // debug!("This ia a debug entry.");
  // }
}
