# Rustic Coffeeshop ☕🌿🏡

HTTP host framework for load balanced workers using AWS SQS and DynamoDB backend.

### Terminology

A coffee shop that has a `Waiter` to take orders, and positive number of `Barista` to
process tickets using the coffee machine. The shop is expected to:

- Listen for incoming requests,
- Convert the requests into tickets on a shared AWS SQS
  queue,
- Have baristas to process the tickets using the coffee machine,
- Put the finished coffee into a AWS DynamoDB table using the SQS id as the key, then
- The barista will shout out the ticket number for the waiter to pick up the order.

The `Shop` is designed to work with load balancers and auto-scaling groups, so that more
`Shop` instances can be deployed to the same cluster to handle the same queue, without
dropping any messages. The load balancing can be performed on the number of messages in
the queue. Depending on the node type for the `Shop`, each `Shop` can have a different
number of baristas within it, but will always have one waiter. Choosing the waiter to
serve incoming requests is the responsibility of the load balancer, and is not part of
this implementation; however as the waiter has very virtually no blocking work to do,
`tokio` alone should be able to handle a large number of requests even if they are not
perfectly balanced across `Shop`.

## `TODO` List

Most issues had been resolved already; bu there are some improvements to be made:

- [ ] Unique errors for `SdkError`
- [ ] Find out why Mac is throwing a `Dispatch Error`.

### Why AWS?

Part of the aim of this project is to provide a single image that can be deployed and scaled easily. It is
a deliberate choice to exclude any kind of cluster level load balancer, database or redis cache that needs
dedicated management. Where possible, such tasks shall be offloaded to tried and tested cloud services,
which AWS is one of the more popular choices.

The chosen services used here all have a free tier that should be sufficient for most testing purposes.
Auto-scaling of EC2 instances will scale costs linearly with your auto-scaling strategy, but as long as
the scale-in policies are aggressive enough, the costs should be kept to a minimum.

### Why the funky name?

I am well known among my colleagues to use an unhealth amount of analogies and metaphors in my explanations.
This way its more fun to understand complex concepts and memorize them.

This particular analogy was actually taken from the official AWS fundamentals course, which I find
very fitting due to this project being highly dependent on AWS services anyway; then the explanation
sort of stuck with the team and we started referring the workers as "baristas" etc which simplified
communication a lot.

So we thought, why not immortalize this analogy in the project name? 🤷‍♂️

## Is a SOAP version available?

Not yet, but the only component that would need to change is the Waiter, which can possibly
be have a feature-flag to switch between REST and SOAP. The rest of the components should be
agnostic to the protocol.
