**Factory method is also known as virtual constructor**

# Intent
This is a creational design pattern that provides an interface for creating objects in a superclass, but allows subclasses
to alter the type of objects that will be created.

# Problem
- You are creating a logistics management application.
- V1 of your app only handles transportation by trucks, so the bulk of your code lives inside the *Truck* class.
- Later on, you decide that you want to incorporate seal logistics into the app, how would the updated code look like?
- In it's current state, most of your code is couples to the *Truck* class.
- Adding a *Ships* into the code would require making changes to the entire code base.
- Moreover, later on, adding more transportation methods would cause you to probaly make more changes again.
- You will end up with a lot or bad, nasty code full of *if* statements that **switch** the apps behaviour depending 
on the class of transportation objects.
