- The visitor patten suggests that you place the new behaviour into a separate class called *visitor* instead of forcing it
  into existing classes.
- The original object that had to perfom the behaviour is now passed to one of the visitor's method as an argument,
  providing the method access to all necessary data contained within the object.
- The visitor class may define a whole set of methods each doing something different on different objects.
- Because each method operates on different objects, you might think that calling the right one will be tricky as
  polymorphism won't wokr here.
- The visitor pattern uses a technique called *Double dispatch* that helps execute a method on a proper object without
  cumbersome conditionals.
- Instead of letting the client select a proper version of the method to call, we delegate this choice to the objects we're
  passing to the visitor as an argument.
- Since the object knows its own class, they'll be able to pick a proper method from the visitor less awkwardly.
- In short, a node accepts a `Visitor` and tells the visitor what method should be executed.
- Notice though that we have changed the node class. But compared to our previous expected change, this change is trivial,
  and allows us to add further behaviour without altering the code once again.
- Now, if we create a common interface for all visitors, all existing nodes can work with any visitor in the app.
- Adding a new export behaviour is now just a matter of implementing a new visitor class.
