Use the Facade pattern when:
1. You need to have a limited but straighforward interface to a complex system.
2. You want to structure a subsystem into layers.
   For example, in our video conversion framework, it can be broken down into two layers for video and audio,
   For each layer, you can create a facade and then make the classes of each layer communicate with each other
   via those facades. This approach is very similar to the *Mediator* pattern.