- Many software designs begin with Factory Method, as it is less complicated and more customizable via subclasses. It then
later on moves to more complex but flexible patterns like AbstractFactory, Prototype or Builder.
- Builder focuses on constructing complex objects step by step. Abstract Factory specializes in creating families of related
objects. Abstract Factory returns the product immediately, whereas Builder lets you run some additional construction steps
before fetching the product.
- You can use Builder when creating complex Composite trees because you can program its construction steps to work 
recursively.
- You can combine Builder with Bridge: the director class plays the role of the abstraction, while different builders act 
as implementations.
- Abstract Factories, Builders and Prototypes can all be implemented as Singletons.