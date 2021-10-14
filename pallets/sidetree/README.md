# Substrate Sidetree Pallet

This Pallet provides the needed functionalities to anchor sidetree based on the Spec https://identity.foundation/sidetree/spec


## Extrinsics

The pallet supports one single extrinsic called `anchor_hash`. This extrinsic must be signed by an account which has a balance on the substrate based chain. The extrinsic needs a `Anchor` struct as the only parameter.

The struct contains two properties

`hash` -> This is the Core Index File Hash which points to the IPFS File

`operations` -> This is the total number of Operaions which are included in the Core Index File

In the current version a Per-Operation Fee is set to a fixed weight of 20_000 per operation. The more operations a Core Index File contains, the extrinsics weight increases multiplied by the amount of operations.

In the next version the `BaseFee` property can be set by an administrative account to modify basefee which will be applied to the extrinsics weight.


## Tests

To execute the tests run

```
cargo test
```
