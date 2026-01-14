- Bridge is usually designed up-front, letting you develop parts of an application independently of each other. 
  On the other hand, Adapter is commonly used with an existing app to make some otherwise-incompatible classes 
  work together nicely.
- Adapter provides a completely different interface for accessing an existing object. On the other hand, with 
  the Decorator pattern the interface either stays the same or gets extended. In addition, Decorator supports 
  recursive composition, which isn’t possible when you use Adapter.
- With Adapter you access an existing object via different interface. With Proxy, the interface stays the same. 
  With Decorator you access the object via an enhanced interface.
- Facade defines a new interface for existing objects, whereas Adapter tries to make the existing interface usable. 
  Adapter usually wraps just one object, while Facade works with an entire subsystem of objects.
- Bridge, State, Strategy (and to some degree Adapter) have very similar structures. Indeed, all of these patterns 
  are based on composition, which is delegating work to other objects. However, they all solve different problems. 
  A pattern isn’t just a recipe for structuring your code in a specific way. It can also communicate to other 
  developers the problem the pattern solves.

