- A Facade class can often be transformed into a Singleton since a single facade object is sufficient in most cases.
- Flyweight would resemble Singleton if you somehow managed to reduce all shared states of the objects to just one 
flyweight object. But there are two fundamental differences between these patterns:
  i. There should be only one Singleton instance, whereas a Flyweight class can have multiple instances with 
    different intrinsic states.
  ii. The Singleton object can be mutable. Flyweight objects are immutable.
- Abstract Factories, Builders and Prototypes can all be implemented as Singletons.

