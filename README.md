
# Connector Hello Source Repository

## Introduction

A Fluvio Connector is a binary built with a connector interface to enable a set
of consistent features.  The features allow for standardized startup and
configuration parameters, as well as the ability to plug in smartmodule
transforms. This interface also allows connector components to be offered in the
Infinyon hub as a component for execution in local or customized container
environments, or, with approval, on Infinyon's cloud platform.

## Repo Organization

This repository is organized with a library crate "ext-lib" and a connector
binary crate "connector-main".

The ext-lib is a simple standalone example integration with the [USGS Earthquake
feed](https://earthquake.usgs.gov/earthquakes/feed/v1.0/geojson.php).

The connector-main takes that simple crate and wraps a Fluvio connector
interface around the library.

## Notes

This example shows a development path to take to integrate general functionality
with a fluvio connector with fairly fine control over the parameters offered.

For simpler http apis, you can altenatively use our generic http-source
connector by filling in a connector configuration file without needing to write
any code. See our docs at https://www.fluvio.io/connectors/inbound/http/ or
https://www.fluvio.io/connectors/outbound/http/.

Or for services which may provide webhook integration, you can also use webook
APIs with the Infinyon cloud webhook gateway at
https://www.infinyon.com/docs/cli/webhook/

