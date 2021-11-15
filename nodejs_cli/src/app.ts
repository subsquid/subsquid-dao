import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { Abi, ContractPromise, BlueprintPromise, CodePromise } from '@polkadot/api-contract';
import {readFileSync} from 'fs';

let wsProvider;
let api;

const node_address = 'ws://127.0.0.1:9944';
// replace with registry proxy real code hash
const rp_codeHash = '0xfc8b0cfb4ad1cfb0d2be9779f1d64e3629ad1a12505171fb1b4c6fe7be0b3ef8';


const endowment = 1230000000000n;
// NOTE The apps UI specifies these in Mgas
const gasLimit = 100000n * 1000000n;
const pay_zero = 0n;

const ep_metaData = JSON.parse(readFileSync('../target/ink/epoch_proxy/metadata.json', 'utf8'));
const epoch_metaData = JSON.parse(readFileSync('../target/ink/epoch/metadata.json', 'utf8'));
const rp_metaData = JSON.parse(readFileSync('../target/ink/registry_proxy/metadata.json', 'utf8'));
const reg_metaData = JSON.parse(readFileSync('../target/ink/registry/metadata.json', 'utf8'));

const deploy = async () => {

// const ep_wasm = readFileSync('../../target/ink/epoch_proxy/epoch_proxy.wasm');
// const epoch_wasm = readFileSync('../../target/ink/epoch/epoch.wasm');
// const reg_wasm = readFileSync('../../target/ink/registry/registry.wasm');
// const rp_wasm = readFileSync('../../target/ink/registry_proxy/registry_proxy.wasm');

// const rp_code = new CodePromise(api, rp_abi, rp_wasm);

// Deploy the WASM, retrieve a Blueprint
// let rp_blueprint;

// createBlueprint is a normal submittable, so use signAndSend
// with an known Alice keypair (as per the API samples)
// const unsub = await rp_code
//   .createBlueprint()
//   .signAndSend(alice, (result) => {
//     if (result.status.isInBlock || result.status.isFinalized) {
//       // here we have an additional field in the result, containing the blueprint
//       rp_blueprint = result.blueprint;
//       unsub();
//     }
//   })

// const rp_blueprint = new BlueprintPromise(api, rp_abi, rp_codeHash);

  // Deploy a contract using the Blueprint

// We pass the constructor (named `new` in the actual Abi),
// the endowment, gasLimit (weight) as well as any constructor params
// (in this case `new (initValue: i32)` is the constructor)
// const unsub = await rp_blueprint.tx
//   .new(endowment, gasLimit, initValue)
//   .signAndSend(alice, (result) => {
//     if (result.status.isInBlock || result.status.isFinalized) {
//       // here we have an additional field in the result, containing the contract
//       rp_contract = result.contract;
//       unsub();
//     }
//   });


// TODO: this
}

// api.tx.contracts
//   .call(<contract addr>, <value>, <max gas>, abi.messages.<method name>(<...params...>))
//   .signAndSend(<keyring pair>, (result: SubmittableResult) => { ... });

// Perform the actual read (no params at the end, for the `get` message)
// (We perform the send from an account address, it doesn't get executed)
// const callValue = await rp_contract
//   .read('get', { pay_value, gasLimit })
//   .send(bob.address);


const get_registry_proxy = async (api: any, abi: any, kp: any, code_hash: any) => {
  const rp_contract = new ContractPromise(api, abi, code_hash);  
  const { gasConsumed, result, output } = await rp_contract.query.get(kp.address, { value: pay_zero, gasLimit });

  // The actual result from RPC as `ContractExecResult`
  console.log(result.toHuman());

  // gas consumed
  console.log(gasConsumed.toHuman());

  // check if the call was successful
  if (result.isOk) {
    console.log('Success', output?.toHuman());
    return output;
  } else {
    console.error('Error', result.asErr);
    throw new Error('Problem!');
  }
}

const registry_make_commitment = async (contract: any, kp: any, from: any, name: string, secret: any) => {
  const { gasConsumed, result, output } = await contract.query.make_commitment(from.address, { pay_zero, gasLimit });
  if (result.status.isInBlock) {
    console.log('in a block');
  } else if (result.status.isFinalized) {
    console.log('finalized');
  }
  // check if the call was successful
  if (result.isOk) {
    console.log('Success', output.toHuman());
    return output;
  } else {
    console.error('Error', result.asErr);
    throw new Error('Problem!');
  }
}

const registry_available = async (contract: any, kp: any, name: string) => {
  const { gasConsumed, result, output } = await contract.query.available(kp.address, { pay_zero, gasLimit });
  if (result.status.isInBlock) {
    console.log('in a block');
  } else if (result.status.isFinalized) {
    console.log('finalized');
  }
  // check if the call was successful
  if (result.isOk) {
    console.log('Success', output.toHuman());
    return output;
  } else {
    console.error('Error', result.asErr);
    throw new Error('Problem!');
  }
}

const registry_commit = async (contract: any, kp: any, commitment: any) => {
  const payment = 100n;
  // query for gasConsumed
  const { gasConsumed, result, output } = await contract.query.commit(kp.address, { pay_zero, gasLimit }, commitment);
  // check if the call was successful
  if (result.isOk) {
    await contract.tx
    .commit({ payment, gasConsumed }, commitment)
    .signAndSend(kp, (result: any) => {
      if (result.status.isInBlock) {
        console.log('in a block');
      } else if (result.status.isFinalized) {
        console.log('finalized');
      }
      // check if the call was successful
      if (result.isOk) {
        console.log('Success', output.toHuman());
        return output;
      } else {
        console.error('Error', result.asErr);
        throw new Error('Problem!');
      }    
    });  
  } else {
    console.error('Error', result.asErr);
    throw new Error('Problem!');
  } 
}

const registry_register = async (contract: any, kp: any, commitment: any) => {
  const payment = 100n;
  // query for gasConsumed
  const { gasConsumed, result, output } = await contract.query.commit(kp.address, { pay_zero, gasLimit }, commitment);
  // check if the call was successful
  if (result.isOk) {
    await contract.tx
    .commit({ payment, gasConsumed }, commitment)
    .signAndSend(kp, (result: any) => {
      if (result.status.isInBlock) {
        console.log('in a block');
      } else if (result.status.isFinalized) {
        console.log('finalized');
      }
      // check if the call was successful
      if (result.isOk) {
        console.log('Success', output.toHuman());
        return output;
      } else {
        console.error('Error', result.asErr);
        throw new Error('Problem!');
      }    
    });  
  } else {
    console.error('Error', result.asErr);
    throw new Error('Problem!');
  } 
}

const register_name = async (api: any, abis: any, from: any, codeHashs: any) => {
  const {rp_abi, reg_abi} = abis;
  const {rp_codeHash} = codeHashs;
  const reg_codeHash = await get_registry_proxy(api, rp_abi, from, rp_codeHash);
  // const reg_contract = new ContractPromise(api, reg_abi, reg_codeHash);
  const name = "myname";
  const secret = 1;
  // const commitment = await registry_make_commitment(reg_contract, from, from, name, secret);
  // await registry_commit(reg_contract, from, commitment);
  // // call available only to print the result - should be true
  // await registry_available(reg_contract, from, name);


}

const main = async () => {
  try {
    var myArgs = process.argv.slice(2);
    console.log('myArgs: ', myArgs);
    // rp_codeHash = myArgs[0];
    wsProvider = new WsProvider(node_address);
    api = await ApiPromise.create({ provider: wsProvider });

    const ep_abi = new Abi(ep_metaData, api.registry.getChainProperties());
    const epoch_abi = new Abi(epoch_metaData, api.registry.getChainProperties());
    const rp_abi = new Abi(rp_metaData, api.registry.getChainProperties());
    const reg_abi = new Abi(reg_metaData, api.registry.getChainProperties());

    const keyring = new Keyring({ type: 'sr25519' });
    // create Alice based on the development seed
    const alice = keyring.addFromUri('//Alice');
    const bob = keyring.addFromUri('//Bob');
    const charlie = keyring.addFromUri('//Charlie');
    const dave = keyring.addFromUri('//Dave');
    const eve = keyring.addFromUri('//Eve');
    const ferdie = keyring.createFromUri('//Ferdie');

    const abis = {rp_abi, reg_abi, ep_abi, epoch_abi};
    const codeHashs = {rp_codeHash};

    await register_name(api, abis, bob, codeHashs);
  } catch (err) {
    console.error('Error', err);    
  }
}

main();

