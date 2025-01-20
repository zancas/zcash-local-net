# Zcash Local Net

## Overview

Utilities that launch and manage Zcash processes. This is used for integration
testing in the development of:

  - lightclients
  - indexers
  - validators

## List of Managed Processes

- Zebrad
- Zcashd
- Zainod
- Lightwalletd

## Prerequisites

Ensure that any binaries managed by this crate are installed on your system.
The binaries can be referenced via $PATH or the path to the binaries can be specified when launching a process.
Each processes `launch` fn and [`crate::LocalNet::launch`] take config structs for defining parameters such as path
locations. See the config structs for each process in validator.rs and indexer.rs for more details.

## Launching multiple processes

See [`crate::LocalNet`].

## Testing

Pre-requisites for running integration tests successfully:
- Build the Zcashd, Zebrad, Zainod and Lightwalletd binaries and add to $PATH.

See [crate::test_fixtures] doc comments for running client rpc tests from external crates for indexer/validator development.

The `test_fixtures` feature is enabled by default to allow tests to run.

