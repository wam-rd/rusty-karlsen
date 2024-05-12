/*!
# `rusty-kaspa WASM32 bindings`

[<img alt="github" src="https://img.shields.io/badge/github-kaspanet/rusty--kaspa-8da0cb?style=for-the-badge&labelColor=555555&color=8da0cb&logo=github" height="20">](https://github.com/kaspanet/rusty-kaspa/tree/master/wasm)
[<img alt="crates.io" src="https://img.shields.io/crates/v/karlsen-wasm.svg?maxAge=2592000&style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/karlsen-wasm)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-kaspa--wasm-56c2a5?maxAge=2592000&style=for-the-badge&logo=docs.rs" height="20">](https://docs.rs/karlsen-wasm)
<img alt="license" src="https://img.shields.io/crates/l/karlsen-wasm.svg?maxAge=2592000&color=6ac&style=for-the-badge&logoColor=fff" height="20">

<br>

Rusty-Kaspa WASM32 bindings offer direct integration of Rust code and Rusty-Kaspa
codebase within JavaScript environments such as Node.js and Web Browsers.

## Documentation

- [**integrating with Kaspa** guide](https://kaspa.aspectron.org/)
- [**Rustdoc** documentation](https://docs.rs/karlsen-wasm/latest/karlsen-wasm)
- [**JSDoc** documentation](https://kaspa.aspectron.org/jsdoc/)

Please note that while WASM directly binds JavaScript and Rust resources, their names on JavaScript side
are different from their name in Rust as they conform to the 'camelCase' convention in JavaScript and
to the 'snake_case' convention in Rust.

## Interfaces

The APIs are currently separated into the following groups (this will be expanded in the future):

- **Transaction API** — Bindings for primitives related to transactions.
- **RPC API** — [RPC interface bindings](rpc) for the Kaspa node using WebSocket (wRPC) connections.
- **Wallet API** — API for async core wallet processing tasks.

## NPM Modules

For JavaScript / TypeScript environments, there are two
available NPM modules:

- <https://www.npmjs.com/package/kaspa>
- <https://www.npmjs.com/package/karlsen-wasm>

The `karlsen-wasm` module is a pure WASM32 module that includes
the entire wallet framework, but does not support RPC due to an absence
of a native WebSocket in NodeJs environment, while
the `kaspa` module includes `websocket` package dependency simulating
the W3C WebSocket and due to this supports RPC.

## Examples

JavaScript examples for using this framework can be found at:
<https://github.com/kaspanet/rusty-kaspa/tree/master/wasm/nodejs>

## WASM32 Binaries

For pre-built browser-compatible WASM32 redistributables of this
framework please see the releases section of the Rusty Kaspa
repository at <https://github.com/kaspanet/rusty-kaspa/releases>.

## Using RPC

**NODEJS:** If you are building from source, to use WASM RPC client
in the NodeJS environment, you need to introduce a global W3C WebSocket
object before loading the WASM32 library (to simulate the browser behavior).
You can the [WebSocket](https://www.npmjs.com/package/websocket)
module that offers W3C WebSocket compatibility and is compatible
with Kaspa RPC implementation.

You can use the following shims:

```js
// WebSocket
globalThis.WebSocket = require('websocket').w3cwebsocket;
```

## Loading in a Web App

```html
<html>
    <head>
        <script type="module">
            import * as karlsen_wasm from './kaspa/karlsen-wasm.js';
            (async () => {
                const kaspa = await karlsen_wasm.default('./kaspa/karlsen-wasm_bg.wasm');
                // ...
            })();
        </script>
    </head>
    <body></body>
</html>
```

## Loading in a Node.js App

```javascript
// W3C WebSocket module shim
// this is provided by NPM `kaspa` module and is only needed
// if you are building WASM libraries for NodeJS from source
// globalThis.WebSocket = require('websocket').w3cwebsocket;

let {RpcClient,Encoding,initConsolePanicHook} = require('./karlsen-rpc');

// enabling console panic hooks allows WASM to print panic details to console
// initConsolePanicHook();
// enabling browser panic hooks will create a full-page DIV with panic details
// this is useful for mobile devices where console is not available
// initBrowserPanicHook();

// if port is not specified, it will use the default port for the specified network
const rpc = new RpcClient("127.0.0.1", Encoding.Borsh, "testnet-10");
const rpc = new RpcClient({
    url : "127.0.0.1",
    encoding : Encoding.Borsh,
    networkId : "testnet-10"
});


(async () => {
    try {
        await rpc.connect();
        let info = await rpc.getInfo();
        console.log(info);
    } finally {
        await rpc.disconnect();
    }
})();
```

For more details, please follow the [**integrating with Kaspa**](https://kaspa.aspectron.org/) guide.

*/

#![allow(unused_imports)]

#[cfg(all(
    any(feature = "wasm32-sdk", feature = "wasm32-rpc", feature = "wasm32-core", feature = "wasm32-keygen"),
    not(target_arch = "wasm32")
))]
compile_error!("`karlsen-wasm` crate for WASM32 target must be built with `--features wasm32-sdk|wasm32-rpc|wasm32-core|wasm32-keygen`");

mod version;
pub use version::*;

cfg_if::cfg_if! {

    if #[cfg(feature = "wasm32-sdk")] {

        pub use karlsen_addresses::{Address, Version as AddressVersion};
        pub use karlsen_consensus_core::tx::{ScriptPublicKey, Transaction, TransactionInput, TransactionOutpoint, TransactionOutput};
        pub use karlsen_pow::wasm::*;

        pub mod rpc {
            //! Kaspa RPC interface
            //!

            pub mod messages {
                //! Kaspa RPC messages
                pub use karlsen_rpc_core::model::message::*;
            }
            pub use karlsen_rpc_core::api::rpc::RpcApi;
            pub use karlsen_rpc_core::wasm::message::*;

            pub use karlsen_wrpc_wasm::client::*;
            pub use karlsen_wrpc_wasm::resolver::*;
            pub use karlsen_wrpc_wasm::notify::*;
        }

        pub use karlsen_consensus_wasm::*;
        pub use karlsen_wallet_keys::prelude::*;
        pub use karlsen_wallet_core::wasm::*;

    } else if #[cfg(feature = "wasm32-core")] {

        pub use karlsen_addresses::{Address, Version as AddressVersion};
        pub use karlsen_consensus_core::tx::{ScriptPublicKey, Transaction, TransactionInput, TransactionOutpoint, TransactionOutput};
        pub use karlsen_pow::wasm::*;

        pub mod rpc {
            //! Kaspa RPC interface
            //!

            pub mod messages {
                //! Kaspa RPC messages
                pub use karlsen_rpc_core::model::message::*;
            }
            pub use karlsen_rpc_core::api::rpc::RpcApi;
            pub use karlsen_rpc_core::wasm::message::*;

            pub use karlsen_wrpc_wasm::client::*;
            pub use karlsen_wrpc_wasm::resolver::*;
            pub use karlsen_wrpc_wasm::notify::*;
        }

        pub use karlsen_consensus_wasm::*;
        pub use karlsen_wallet_keys::prelude::*;
        pub use karlsen_wallet_core::wasm::*;

    } else if #[cfg(feature = "wasm32-rpc")] {

        pub use karlsen_rpc_core::api::rpc::RpcApi;
        pub use karlsen_rpc_core::wasm::message::*;
        pub use karlsen_rpc_core::wasm::message::IPingRequest;
        pub use karlsen_wrpc_wasm::client::*;
        pub use karlsen_wrpc_wasm::resolver::*;
        pub use karlsen_wrpc_wasm::notify::*;
        pub use karlsen_wasm_core::types::*;

    } else if #[cfg(feature = "wasm32-keygen")] {

        pub use karlsen_addresses::{Address, Version as AddressVersion};
        pub use karlsen_wallet_keys::prelude::*;
        pub use karlsen_wasm_core::types::*;

    }
}
