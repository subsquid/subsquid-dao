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

pub fn set_link(&mut self, name: Hash, link: String)
This on chain function will register IPNS link for a registered name, you need to be the owner to call this

### get link

pub fn get_link(&self, name: Hash) -> Option<String>
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
