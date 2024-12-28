# Rustic Coffeeshop ☕🌿🏡

HTTP host framework for load balanced workers using AWS SQS and DynamoDB backend.


### Why AWS?

Part of the aim of this project is to provide a single image that can be deployed and scaled easily. It is
a deliberate choice to exclude any kind of cluster level load balancer, database or redis cache that needs
dedicated management. Where possible, such tasks shall be offloaded to tried and tested cloud services,
which AWS is one of the more popular choices.

The chosen services used here all have a free tier that should be sufficient for most testing purposes.
Auto-scaling of EC2 instances will scale costs linearly with your auto-scaling strategy, but as long as
the scale-in policies are aggressive enough, the costs should be kept to a minimum.

## Why the funky name?

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
