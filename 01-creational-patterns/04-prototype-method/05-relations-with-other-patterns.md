- Many software designs begin with Factory Method, as it is less complicated and more customizable via subclasses. It then
later on moves to more complex but flexible patterns like AbstractFactory, Prototype or Builder.
- Abstract Factory classes are often based on a set of Factory Methods, but you can also use Prototype to compose the 
methods on these classes.
- Prototype can help when you need to save copies of Commands into history.
- Designs that make heavy use of Composite and Decorator can often benefit from using Prototype. Applying the pattern 
lets you clone complex structures instead of re-constructing them from scratch.
- Prototype isn’t based on inheritance, so it doesn’t have its drawbacks. On the other hand, Prototype requires a 
complicated initialization of the cloned object. Factory Method is based on inheritance but doesn’t require 
an initialization step.
- Sometimes Prototype can be a simpler alternative to Memento. This works if the object, the state of which you want 
to store in the history, is fairly straightforward and doesn’t have links to external resources, or the links are easy 
to re-establish.
- Abstract Factories, Builders and Prototypes can all be implemented as Singletons.
