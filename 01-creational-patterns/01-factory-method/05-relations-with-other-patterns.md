- Many software designs begin with Factory Method, as it is less complicated and more customizable via subclasses. It then
later on moves to more complex but flexible patterns like AbstractFactory, Prototype or Builder.
- AbstractFactory classes are often based on a set of Factory methods but you can also use Prototype to compose the methods on
these classes.
- You can use FactoryMethod along with Iterator to let collection subclasses return different types of iterators that are
compatible with the collections.
- Unlike Prototype that isn't based on inheritance, thus doesn't have the usual drawbacks of inheritance, but is a bit
more complicated as it requires initialization of the cloned object, FactoryMethod is based on inheritance but does not
require an initialization step.
- FactoryMethod is a specialization of TemplateMethod. Similarly, a FactoryMethod may serve as a step in a large TemplateMethod.