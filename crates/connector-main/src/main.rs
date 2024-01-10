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

#[connector(config)]
#[derive(Debug)]
pub(crate) struct CustomConfig {
    /// Polling interval in seconds
    interval_sec: u32,

    /// Generic parameter capable of accepting a secret
    pub secret_example_param: Option<SecretString>,
}

#[connector(source)]
async fn start(config: CustomConfig, producer: TopicProducer) -> Result<()> {
    println!("Starting source connector with {config:?}");
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
    let slen = secret_param.chars().count();
    println!("secret_example_param is {slen} chars long");

    let delay = std::time::Duration::from_secs(config.interval_sec.into());
    let mut first_update = true;
    loop {
        if first_update {
            first_update = false;
        } else {
            // apply the delay from the config
            info!("Waiting {} for next update", config.interval_sec);
            sleep(delay);
        }

        // Read data from the USGS endpoint
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
        producer.flush().await?;
    }
}


