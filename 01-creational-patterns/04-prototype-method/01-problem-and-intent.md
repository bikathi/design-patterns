# Intent
- This is a creational pattern that lets you copy existing objects without making your code dependent on their classes.

# Problem
- Say you have an object and you want to make an exact copy of it. How would you do it?
- First, you have to create a new object of the same class, then you go through all the fields of the original object, copying
their values over to the new object.
- That works but there is a catch: Not all objects can be copied that way because some of the objects fields may be private and
not visible from outside the object itself.
- Furthermore, since you have to know the object's class to create a duplicate, your code becomes dependent on that class.
- Even more? Sometimes, you only know the interface that the object follows, but not it's concrete class, when, for example,
a parameter in a method accepts any objects that follow the same interface.