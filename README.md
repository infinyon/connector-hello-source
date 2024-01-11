
# Connector Hello Source Repository

## Introduction

A Fluvio Connector is a binary built with a connector interface to enable a set
of consistent features.  The features allow for standardized startup and
configuration parameters, as well as the ability to plug in smartmodule
transforms. This interface also allows connector components to be optionally
published in the Infinyon hub as a component for execution in local or
other user container environments, or with approval, within Infinyon's cloud
platform.

A "source" and "sink" refers to how a connector interacts with a fluvio cluster.
A "source" connects to an external data source and publishes records into a
fluvio topic. A "sink" recives new events from a fluvio topic, and sends the
data to an external data store.

## Repo Organization

This repository is organized with a library crate "ext-lib" and a connector
binary crate "connector-main":

* [external-lib](crates/external-lib) is a simple standalone example integration with the [USGS Earthquake
feed](https://earthquake.usgs.gov/earthquakes/feed/v1.0/geojson.php). A separate crate is not required by the Fluvio connector interface, but the arrangement is often convenient for testing and development.

* [connector-main](crates/connector-main) is the glue logic that joins the simple crate with the Fluvio connector
interface.

## Build and Test

The external-lib can be built and tested using standard cargo commands.

At the top cargo workspace level:
```
cargo build -p external-lib  # to just build the external-lib crate
cargo test -p external-lib   # this will build and test the crate
```
Or in the crate directory:
```
cd crates/external-lib
cargo build
cargo test
```

Building the connector can be performed using the connector development kit cli
tool `cdk` which comes with a standard fluvio install. Cdk does take care of
some common uses, such as building for multiple targets, and working with
the standard connector configuration files.

To build with cdk:
```
cd crates/connector-main
cdk build
```

Or you can still build with cargo:
```
cd crates/connector-main
cargo build
```

To test cdk with a given connector config file. This outputs all log outputs
directly to the terminal.

```
cd crates/connector-main
cdk test --config config.yaml
```

The connector produces records into the `output-topic`. To check the result:

```
fluvio consume output-topic -B -O json
```

To deploy and shutdown a connector with cdk. A connector deployed this way
will be running in a local process with log outputs going to a file, but able to
be examined conveniently with `cdk deploy log`.

```
cd crates/connector-main
cdk deploy start --config config.yaml
cdk deploy log --config config.yaml
cdk deploy shutdown --config config.yaml
```

For connector deployments in the Infinyon cloud see:
https://fluvio.io/connectors/cloud-connectors/


## Using Secrets

This sample connector is configured with an optional example secret parameter.
to test or run with the secret, an alternate configuration is used, and the
secret needs to be provisioned to `cdk test` or `cdk deploy` with the `-s` parameter.

```
cd crates/connector-main
cdk test --config config-with-secret.yaml -s secret.txt
cdk deploy start --config config-with-secret.yaml -s secret.txt
```


## Notes

This example shows a development integration of a generic crate to wrap a
fluvio connector interface. This integration gives close control over the
parameters offered as well as how the connector executes.

For simpler uses, you can used existing parameterized connectors from the hub.
For example with many http apis, you can altenatively use the Infinyon generic
http-source connector by filling in a connector configuration file without
needing to develop a custom connector. In the case of the USGS feed, an alternate
way to process the data would be to use the generic http source connector with
smartmodules to process the geojson data into a record stream.

See our docs at:
* https://www.fluvio.io/connectors/inbound/http/ or
* https://www.fluvio.io/connectors/outbound/http/.

For services which may provide webhook integration, you can also use webook
APIs with the Infinyon cloud webhook gateway at:
* https://www.infinyon.com/docs/cli/webhook/

See a list of available connectors at
 * https://fluvio.io/connectors/cdk/github-examples/


## Collaborate

* Join the [Fluvio Community on Discord](https://discordapp.com/invite/bBG2dTz) to ask questions, promote your connector, or simply say hello.
