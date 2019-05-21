[![Build Status](https://travis-ci.org/sunrise-choir/ssb-patchql.svg?branch=master)](https://travis-ci.org/sunrise-choir/ssb-patchql)
[![Build status](https://ci.appveyor.com/api/projects/status/w1c7n87463g99vls/branch/master?svg=true)](https://ci.appveyor.com/project/pietgeursen/ssb-patchql/branch/master)
# ssb-patchql

> (:construction: work in progress :construction: ) An example graphql api suitable for making ssb apps similar to patchwork or patchbay

## Heads Up:

- this is still a wip, we're still learning and working out the best shape for this api to take.

## Intention

- I'd like to see 15-20 new ssb apps by 2025. Doing this by:
  - lowering the technical barrier to entry by using industry standard technologies.
  - making well documented, easy to understand code, that's tested and reliable

- there's a high barrier to entry in the js ssb stack, even for just building a front-end client. Much of the tech stack is "Mad Science" / experimental.
  - you need to learn pull-streams, flume-db, ssb-msg-schema, ssb-server api, ...
  - drawbacks of the high barrier:
    - only a few people have the time to invest in learning all the tech in the stack
    - those few people become accidental maintainers / wield power in the community because they shape the apps that people use / single points of failure / bottlenecks for progress. (Not meant to malign any people who have done great work eg, Matt, Mix, Dominic)

## Tech Decisions

### Why the `process` mutation?

This db will lag behind the offset log and needs calls to `process` to bring the db up to date. At first this might seem annoying and that the db should do this automatically. But this is a conscious design decision to give the app control of when cpu is used. This is very important on resource constrained devices, or even just when starting up the app. This is a major pain point in the javascript flume-db implementation that we're avoiding by doing this.

### Why sql?

- SSB data is highly relational. It suits a relational db very well.
- Each person has a db that only contains data from their own social network (and not all the data of the entire network like in a centralised system) we don't have to be able to scale to millions or billions of users.

## Graphql Schema

[graphql schema](/schema.graphql) lives here.

Example Queries:

```graphql

```

## Database schema

![schema](/docs/images/ssb-patchql.jpg)

## Todos

## Development

### Install the graphql-cli

With node / npm installed:

`$ npm install -g graphql-cli`

### Generate a new `schema.graphql`

Copy `.env_example` to a file called `.env` and edit parameters appropriately for your environment.

Start the server using `$ cargo run`

and in another terminal:

`$ graphql get-schema`

### Lint the generated schema for errors

`$ graphql lint` (and press enter to accept the default option)

