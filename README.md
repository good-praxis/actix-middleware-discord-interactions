# Discord Interactions Middleware

## Features

- Respond to ping requests by the Discord server [ ]
- Process interaction data [ ]

_Please Note_: This middleware does not verify the ed25519 signature, as [expected by the discord server](https://discord.com/developers/docs/interactions/receiving-and-responding#security-and-authorization). This middleware is intended to be used alongside [actix-middleware-ed25519-authentication](https://crates.io/crates/actix-middleware-ed25519-authentication). Alternatively, you can write your own authentication middleware and apply it to your app through `warp_fn`.
Eventually the ed25519-authentication should be integrated in this middleware behind a feature flag.

## Usage

tbd.

## Acknowledgements

For a more batteries-included library that handles discord interactions, check out [rusty-interactions](https://github.com/0x2b00b1e5/rusty-interaction). Their feature scope is more extensive than this crate.

Thanks to these authors for their writeups that helped with development of this crate:

- [Demystifying Actix Web Middleware](https://dev.to/dimfeld/demystifying-actix-web-middleware-3lef) by Daniel Imfeld
