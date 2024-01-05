
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
flvuio topic. A "sink" recives new events from a fluvio topic, and sends the
data to an external data store.

## Repo Organization

This repository is organized with a library crate "ext-lib" and a connector
binary crate "connector-main".

The ext-lib is a simple standalone example integration with the [USGS Earthquake
feed](https://earthquake.usgs.gov/earthquakes/feed/v1.0/geojson.php).

The connector-main takes that simple crate and wraps a Fluvio connector
interface around the library.

## Notes

This example shows a development integration of a generic crate to wrap a
fluvio connector interface. This integration gives close control over the
parameters offered as well as how the connector executes.

For simpler uses, you can used existing parameterized connectors from the hub.
For example with many http apis, you can altenatively use the Infinyon generic
http-source connector by filling in a connector configuration file without
needing to develop a custom connector. See our docs at
https://www.fluvio.io/connectors/inbound/http/ or
https://www.fluvio.io/connectors/outbound/http/.

For services which may provide webhook integration, you can also use webook
APIs with the Infinyon cloud webhook gateway at
https://www.infinyon.com/docs/cli/webhook/

Connectors can be browsed at TBD.
