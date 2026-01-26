# Intent
Is a structural desgin pattern that lets you split a large class or a subset of closely related classes into two separate
hierarchies that can be developed independently of each other:
  1. abstraction
  2. implementation

# Problem
- Say you have a *Shape* class with a pair of subclasses: *Circle* and *Square*.
- You want to extend this class hierarchy to incorporate colors, so you plan to create *Red* and *Blue* shape
  subclasses.
- But, since you already have two subclasses, you'll need to create four class combinations such as *BlueCirle* and
  *RedSquare*
- Adding new shapes and colors to the hierarchy will make it grow exponentially.