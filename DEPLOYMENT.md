# Deployment

## Building contracts

Building contract is integrated into a docker image, see docker folder.
A script to run that instance and build the contract is provided by `build_contract_with_docker.sh`, make sure you have docker installed and functional.
Calling the script will generate inside target/ink folder the files: ssindexer.contract, ssindexer.wasm and metadata.json

## Running a local contract based node

Another docker image is provided to run a local node and that node is configured using docker compose. Make sure you have docker compose installed and functional. Just run `docker-compose up` and your node should be running.

## Interaction with the contract

A general purpose ui interface is provided by <https://paritytech.github.io/canvas-ui>. Once there make sure you select local node and grant any permissions if asked.

## Contract Instance creation

Follow the instructions on <https://docs.substrate.io/tutorials/v3/ink-workshop/pt1/#1-upload-contract-code> but instead use the compiled ssindexer.contract on target/ink folder
