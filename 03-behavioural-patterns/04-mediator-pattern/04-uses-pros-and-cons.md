# Use the mediator pattern when:
1. It's hard ot change some of the classes because they are tightly coupled to a bunch of other classes.
2. You can't reuse a component in a different program because it is too dependennt on other components.
3. You find yourself creating a ton of  component subclasses just to reuse some basic behaviour in various contexts.

# Pros
- Single Responsibility Principle as you extract the communication between various components into a single place, making
  it easier to comprehend and maintain.
- Open/Closed principle - you can introduce new mediators without having to change the actual components.
- You reduce coupling between various components of a program.
- You can reuse individual components more easily.

# Cons
Over time, a mediator can evolve into a God object.