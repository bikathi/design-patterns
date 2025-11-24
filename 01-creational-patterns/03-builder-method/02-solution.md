- The builder method suggests that you extract the object construction code out of its own class and move it to a separate
object called a *builder*.
- In the code, you organize the object construction into a set of steps e.g *buildWalls*, *buildDoor* e.t.c, such that to
create an objects, you go through a series of steps.
- Importantly, some steps can be skipped during object construction, allowing you to only call the steps that are necessary
to produce a particular configuration of an object.
- Some of the construction steps might require different implementations when you need to build various representations of
the product. For example, walls of a cabi may be build of wood, but castle walls must be built with stone.
- In this case you create several builder classes that implement the same building steps, but in a different manner.
- Then, you can use those builders in the construction process to produce different kinds of objects.
- For example, you might have a builder that builds everything from wood and glass, a second one that builds everything with
stone and iron, and a third one that uses gold and diamonds.
- By calling the same set of steps, you get a regular house from the first builder, a small castel from the second, and a 
palace from the third.
- However, this would only work if the client code thatc alls the building steps is able to interact with builders using a 
common interface.

# Director
- You can go further and extract a series of calls to the builder steps you use to construct a product into a separate class
called a *director*.
- The director defines the order in which to execute the building steps, while the builder provides the implementation for
those steps.
- The director class isn't strictly necessary to have as you can always call the building steps in a specific order directly
from the client code.
_ The director class, however, might be a good place to put various construction routines so you can reuse them across
your program.
- Additionally, the director class completely hides the details of product construction from the client code.
- The client only needs to accosiate a builder with a director, lauch the construction with the director, and get the result
from the builder.
