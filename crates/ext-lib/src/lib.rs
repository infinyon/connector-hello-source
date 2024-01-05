use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use geojson::Feature as GeoFeature;
use geojson::JsonValue;

/// updated once per minute
const ENDPOINT: &str = "https://earthquake.usgs.gov/earthquakes/feed/v1.0/summary/all_hour.geojson";

#[derive(Debug, Deserialize, Serialize)]
pub struct UsgsFeatureCollectionExample {
    #[serde(rename = "type")]
    pub type_: String, // expect FeatureCollection
    pub metadata: std::collections::HashMap<String, JsonValue>,
    pub features: Vec<GeoFeature>,
}

impl UsgsFeatureCollectionExample {
    pub async fn update() -> Result<Self> {
        let body = reqwest::get(ENDPOINT)
            .await
            .map_err(|e| anyhow!("Couldn't access {ENDPOINT}, error {e}"))?
            .text()
            .await
            .map_err(|e| anyhow!("Couldn't read body, error {e}"))?;
        UsgsFeatureCollectionExample::parse(&body)
    }

    pub fn parse(geojson_str: &str) -> Result<Self> {
        let data: UsgsFeatureCollectionExample = serde_json::from_str(&geojson_str)
            .map_err(|e| anyhow!("Couldn't parse geojson, error {e}"))?;
        Ok(data)
    }

    pub fn metadata(&self, key: &str) -> Option<JsonValue> {
        self.metadata.get(key).cloned()
    }

    /// grab generated time from data
    pub fn generated(&self) -> Result<i64> {
        let Some(generated) = self.metadata("generated") else {
            return Err(anyhow!("no generated key"));
        };
        let Some(generated) = generated.as_i64() else {
            return Err(anyhow!("couldn't convert \"generated\" value to i64 {}", generated));
        };
        Ok(generated)
    }

    /// check consistency of data
    pub fn valid(&self) -> bool {
        if self.type_ != "FeatureCollection" {
            return false;
        }
        if !self.metadata.contains_key("count") {
            return false;
        }
        let count = self.metadata("count").map_or(0i64, |val| {
            val.as_i64().unwrap_or_default()
        });
        let count_ok = count == self.features.len() as i64;

        let generated = self.generated();

        count_ok && generated.is_ok()
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use anyhow::anyhow;
    use reqwest;

    use super::*;

    const SAMPLE_JSON: &str = "tests/all_hour.geojson";

    #[test]
    fn parse() -> anyhow::Result<()> {
        let geojson = std::fs::read_to_string(SAMPLE_JSON)?;
        let data = UsgsFeatureCollectionExample::parse(&geojson)?;
        println!("{data:?}");

        assert_eq!(data.type_, "FeatureCollection");
        assert!(data.valid());

        let Some(_last_updated) = data.metadata("generated") else {
            return Err(anyhow!("no generated key"));
        };
        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn run_update() -> anyhow::Result<()> {
        let data = UsgsFeatureCollectionExample::update().await?;
        println!("{data:?}");
        Ok(())
    }


    #[ignore]
    #[tokio::test]
    async fn get_endpoint_raw_text() -> anyhow::Result<()> {
        let body = reqwest::get(ENDPOINT)
            .await
            .map_err(|e| anyhow!("Couldn't access {ENDPOINT}, error {e}"))?
            .text()
            .await
            .map_err(|e| anyhow!("Couldn't read body, error {e}"))?;

        // write file as sample fixture
        let mut file = std::fs::File::create(SAMPLE_JSON)?;
        file.write_all(&body.as_bytes())?;
        Ok(())
    }
}
