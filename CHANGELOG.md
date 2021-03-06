# Changes

## 2021 Oct 22

### valid

pub fn valid(&self, name: String) -> bool
This off chain call will check if the name you want to register is valid by checking some rules. Currently checking only for a max name size of 256, could make this configurable.

### get_hash

pub fn get_hash(&self, name: String) -> Hash
This off chain call will hash the name you want to register for further use on other functions

### avaliable

pub fn available(&self, name: String) -> bool
This off chain call will check if the name you want to register is occupied

### make_commitment

In order to prevent front running issues first we need to make commitment to a name.
Making an off chain rpc call to:
pub fn make_commitment(&self, name: String, owner: AccountId, secret: u32) -> Hash
will generate a Hash and only the caller knows the data that leads to that Hash

### commit

pub fn commit(&mut self, commitment: Hash)
will submit your commitment on chain with some configurable fee which will be valid for a configurable number of blocks.

### register

pub fn register(&mut self, name: String, from: AccountId, duration: u32, secret: u32)
After your commitment is saved on chain you can call register without front running issues
In order to clean up data the registration will be valid for a certain duration period.
Payment of the registration is required

### unregister

pub fn unregister(&mut self, name: Hash)
If you are the owner of a registered name you can unregister it. You pass the Hash of the name calculated by calling function get_hash

### set link

pub fn set_link(&mut self, name: Hash, link: Hash)
This on chain function will register IPNS link for a registered name, you need to be the owner to call this

### get link

pub fn get_link(&self, name: Hash) -> Option<Hash>
This off chain function will retrieve the IPNS link associated with registered name.

### set capabilities

pub fn set_capability(&mut self, name: Hash, property: Hash, value: String)
This could change, currently it is a general purpose key value pair, you should first get the hash of the property by calling function get_hash

### get capabilities

pub fn get_capabilities(&self, name: Hash) -> BTreeMap<Hash, String>
This off chain function will retrieve the capabilities associated with registered name.

### delegate

pub fn delegate(&mut self, name: Hash, from: AccountId)
Make a payment transaction which will be associated with the 'from' account for the registered name

## 2021 Oct 31

### Undelegate

pub fn undelegate(&mut self, name: Hash)
Cancel previous delegation which will withdraw the delegated amount. TODO: soon epoch and reward manager are integrated should apply those rules in the process

### Get delegate

        pub fn get_delegate(
            &self,
            investor: AccountId,
            name: Hash,
        ) -> Option<(Balance, BlockNumber)>

Retrieve delegation information, balance delegated and on what blockNumber was delegated

### Subscribe

pub fn subscribe(&mut self, name: Hash, from: AccountId)
User subscribe to indexer name by transfering subscription fee

### Unsubscribe

pub fn unsubscribe(&mut self, name: Hash)
User unsubscribe from indexer name and withdraw remaining balance. TODO: currently check for a min threshold period which is fixed currently and when parametrization on dao manager, together with epoch manager, are integrated should get from there.

### Get subscription

pub fn get_subscription(&self, name: Hash, from: AccountId) -> Option<SubscriberData>
Retrieve subscription data for a given user and indexer name. Data is currently remaining balance and blockNumber for last claimed fees

### Claim fee

pub fn claim_fees(&mut self, name: Hash)
Indexer claim fees for one registered name for used services from subscriber. TODO: currently epoch and reward are not integrated, so delegation is not considered yet.

## 2021 Nov 7

### Indexer refactor

The indexer is now split into several smaller contracts with a proxy each one. Each proxy stores the hash code of current used contracted.

Link and Capabilities are now part of indexer meta contract with Delegation into delegation contract and subscription on it's own contract with same name.

### Epoch manager

pub fn get_current_epoch(&self) -> u32
Get current epoch since genesis

pub fn get_current_block(&self) -> u32
Get current block in current epoch

pub fn get_period_length(&self) -> BlockNumber
Get length in block for each epoch

pub fn get_offset(&self) -> BlockNumber
Get an offset to shift from genesis

pub fn get_current_epoch_since(&self, since: BlockNumber) -> u32
Calculate diference in epoch from current Height to since param