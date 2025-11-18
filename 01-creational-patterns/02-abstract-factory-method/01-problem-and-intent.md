# Intent
Abstract Factory is a creational design pattern that lets you producde families of related objects without specifying their
concrete classes.
It is an extension of the [Factory Pattern](01-creational-patterns/01-factory-method/01-problem-and-intent.md) wherein we add
a lot of factory method.

# Problem
- Imagine you are creating a furniture shop simulator with code consists of classes that represent:
  1. A family of related products e.g. *Chair*, *Sofa*, *CoffeeTable*
  2. Several variants of each family. For example, products *Chair*, *Sofa*, *CoffeeTable* are available in the variants
  *Modern*, *Victorian*, *ArtDeco*
- You need a way to create individual furniture objects so that they match other objects of the same family.
- Also, you don't want to change existing code when adding new products or families of products to the program. You wouldn't
want to change the core code each time it happens.