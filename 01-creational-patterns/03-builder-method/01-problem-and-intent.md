# Intent
- This is a creational pattern that lets you construct complex objects step by step.
- The pattern allows you to producr different types and representations of an object using the same construction code.

# Problem
- You have a complex object that requires laborious step-by-step initialization of many fields and nested objects.
- We have the initialization code buried inside some gigantic monstrous constructor with lots of parameters, or even worse,
scattered all over the client code.
- For example, let's think about how to create a *House* object.
- To build a simple house, you need to construct four walls and a floor, install a door, fit a pair of windows, and build
a roof.
- But what if you want a bigger, brighter house, with a backyard and other goodies like a heating system, plumbing and
electrical wiring?
- A simple way is to extend the base *House* class and create a set of subclasses to cover all combinations of parameters.
- Eventually, you will end up with a large number of subclasses. Any new parameter such as the porch style will require
growing this hierarchy even more.
- Alternatively, you can create a giant constructor right int he base *House* class with all possible parameters that control
the house.
- With this approach, you don't need subclasses, but in many casses, when calling the constructor, many of the parameters will
not be used.
- You now end up with a constructor calls that look ugly when a house constuction for example, doesn't need a pool.
