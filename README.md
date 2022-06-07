# Currents Graph on Solana POC

## Proof of concept for: 

- Anchor framework application to the graph structure
- Program Derived Addresses for graph entities
- Search graph entities by memcmp

## Entities

- user node 
- blog node 
- subscription edge

## API

- create a user
- create a blog
- link a user to a blog with a Subscription edge

## Getters POC

- search for a user/blog/subscription by memcmp
- get a user/blog/subscription by known key (PDA)

## To run

`anchor test`