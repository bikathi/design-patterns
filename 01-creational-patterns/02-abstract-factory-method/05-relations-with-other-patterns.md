- Many software designs begin with Factory Method, as it is less complicated and more customizable via subclasses. It then
later on moves to more complex but flexible patterns like AbstractFactory, Prototype or Builder.
- Builder focuses on constructing complex objects step by step. Abstract Factory specializes in creating families of related
objects. Abstract Factory returns the product immediately, whereas Builder lets you run some additional construction steps
before fetching the product.
- AbstractFactory classes are often based on a set of Factory methods but you can also use Prototype to compose the methods on
these classes.
- Abstract Factory can serve as an alternative to Facade when you only want to hide the way the subsystem objects are created from the client code.
- You can use Abstract Factory along with Bridge. This pairing is useful when some abstractions defined by Bridge can only
work with specific implementations. In this case, Abstract Factory can encapsulate these relations and hide the complexity
from the client code.
- Abstract Factories, Builders and Prototypes can all be implemented as Singletons.

