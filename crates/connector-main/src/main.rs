use anyhow::anyhow;
use fluvio::{RecordKey, TopicProducer};
use fluvio_connector_common::{
    connector,
    Result,
    secret::SecretString,
};
use std::thread::sleep;
use tracing::info;

use external_lib::UsgsFeatureCollectionExample;

/// Configuration for the custom source connector.
#[connector(config)]
#[derive(Debug)]
pub(crate) struct CustomConfig {
    /// Polling interval in seconds
    interval_sec: u32,

    /// Generic parameter capable of accepting a secret
    pub secret_example_param: Option<SecretString>,
}

/// The custom source connector polls an external API and produces records.
///
/// It connects to a Fluvio stream and periodically polls the external API.
/// The received records are produced into the stream.
#[connector(source)]
async fn start(config: CustomConfig, producer: TopicProducer) -> Result<()> {
    // Print the loaded configuration
    println!("Starting source connector with {config:?}");
    // Validate interval is >= 60 seconds
    if config.interval_sec < 60 {
        return Err(anyhow!("interval_sec: minimum is 60 seconds"));
    }

    // resolve is parameter is optionally provided
    let secret_param = if let Some(secret) = config.secret_example_param {
        // resolve the secret
        secret.resolve().unwrap_or_default()
    } else {
        // parameter not supplied
        String::new()
    };
    // Print secret length
    let slen = secret_param.chars().count();
    println!("secret_example_param is {slen} chars long");

    // Calculate polling interval
    let delay = std::time::Duration::from_secs(config.interval_sec.into());
    // Poll loop
    let mut first_update = true;
    loop {
        // Handle first vs subsequent updates
        if first_update {
            first_update = false;
        } else {
            // apply the delay from the config
            info!("Waiting {} for next update", config.interval_sec);
            sleep(delay);
        }

        // Read data from external API(USGS) endpoint
        let Ok(data) = UsgsFeatureCollectionExample::update().await else {
            info!("Failed update");
            continue;
        };
        info!("Source updated");

        // the USGS data contains a vec of earthquake reports as a geojson
        // Feature for each update, send each report individaully to the topic
        for rec in data.features.iter() {
            let rec: String = rec.to_string();
            producer.send(RecordKey::NULL, rec).await?;
        }
        // Flush records
        producer.flush().await?;
    }
}


