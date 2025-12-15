- The Prototype pattern delegates the cloning process to the actual objects that are being clonsed.
- The pattern declaresa a common interface for all objects that support cloning. This interfcae lets you clone an object
without coupling your code to the class of the object.
- Usually, such an interface contains a single *clone()* method.
- The implementation of the *clone* method is very similar in all classes. It creates a new object of the current class, and
carries over all of the field values of the old object into the new onw.
- You can even copy private fields because most languages let objects access private fields of other objects that belong to the
same class.
- An object that supports cloning is called the *prototype*.
- The idea is this: you create a set of objects, configured in different ways. When you need an object like the one you've
configured, you just clone a prototype instead of constructing a new object from scratch.
> When your objects have dozens of fields and hundreds of possible configurations, cloning them might serve as an alternative
to subclassing.