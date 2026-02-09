- You can use Builder when creating complex Composite trees because you can program its construction 
  steps to work recursively.
- Chain of Responsibility is often used in conjunction with Composite. In this case, when a leaf component 
  gets a request, it may pass it through the chain of all of the parent components down to the root of the object tree.
- You can use Iterators to traverse Composite trees.
- You can use Visitor to execute an operation over an entire Composite tree.
- You can implement shared leaf nodes of the Composite tree as Flyweights to save some RAM.
- Composite and Decorator have similar structure diagrams since both rely on recursive composition to organize 
  an open-ended number of objects.