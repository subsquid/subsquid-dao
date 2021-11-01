# Indexer contract

## Subsquid Indexer RDS (Registry, delegation and subscription) contract

Basic registry contract to register an indexer. The registry will be approved by the governor. For the sake of simplicity for the MVP, we will be mapping each indexer with a unique name and the amount staked ( which >= min_amount to register)and a fee per day required to be paid by the consumer for availing service.

Consumers can subscribe to the service of an indexer by paying fees which will be locked to a certain period.

### Functionalities to implement

* Function to register an indexer with a unique name, IPNS link with details on indexer capabilities ( Need to finalise a formal structure for this), address of the indexer through governor
* Function to deregister an indexer and return stake back to the indexer address (This can be done by the owner of the indexer)
* Function to view details of an indexer with by giving indexer id / name
After the indexer is registered through governor, Functionality to receive the minimum threshold which has to be staked in the contract to activate the indexer to receive requests, rewards etc
* Functionality to delegate tokens to an indexer by an investor
* Functionality to initiate withdraw delegated tokens  by an investor (Funds will be locked till a min threshold of epochs)
* Functionality to view delegate tokens  by an investor to an index
* Functionality for a consumer to subscribe to an indexer by paying a fees for a time period according to the fees proposed by the indexer
* Functionality for a consumer to cancel subscription to an indexer and get back unutilised funds
* Function to view details of consumer subscription
* Functionality to claim fees by indexer for a consumer subscription
