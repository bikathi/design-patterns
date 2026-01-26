- Bridge is usually designed up-front, letting you develop parts of an application independently of each other. 
  On the other hand, Adapter is commonly used with an existing app to make some otherwise-incompatible classes 
  work together nicely.
- Bridge, State, Strategy (and to some degree Adapter) have very similar structures. Indeed, all of these patterns 
  are based on composition, which is delegating work to other objects. However, they all solve different problems. 
  A pattern isn’t just a recipe for structuring your code in a specific way. It can also communicate to other 
  developers the problem the pattern solves.
- You can use Abstract Factory along with Bridge. This pairing is useful when some abstractions defined by Bridge 
  can only work with specific implementations. In this case, Abstract Factory can encapsulate these relations and 
  hide the complexity from the client code.
- You can combine Builder with Bridge: the director class plays the role of the abstraction, while different 
  builders act as implementations.



