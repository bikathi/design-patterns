# Intent
Is a structural design pattern that lets you compose objects into tree structures and then work with these structures
as if they were individual objects.

# Problem
- Using the composite pattern only makes sense when the core model of your app can be represented as a tree.
- Imagine you have two types of objects: *Products* and *Boxes*.
- A *Box* can contain serveral *Products* as well as a number of smaller *Boxes*.
- These little *Boxes* can also hold some *Products* or even smaller *Boxes* and so on.
- Say you create a system to stores orders using these classes.
- An order could contain products without any wrapping, as well as boxes stuffed with products, and other boxes.
- How would you determine the total proce of such an order?
- The naive and direct approach is to unwrap all the boxes and go over all the products, calculating the total.
- In a real world, that's possible. But in code, this is not straighforward.