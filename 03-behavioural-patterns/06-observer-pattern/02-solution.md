- The object has some interesting state, often called a *subject*.
- Since the object is supposed to notify other objects about changes to its state, we call it a *publisher* too.
- All other objects that want to track changes to the publisher's state are called *subscribers*.
- The observer patten suggests that you add a subscription mechanism to the publisher class, so individual objects can
  subscirve or unsubscrive from a stream of events coming from that publisher.
- In reality, this mechanism consists of:
  1. An aray field for storing a list of references to a subsciber.
  2. Seevral public methods which allow subscribers to add or remove themselves from that list.
- Whenever an important event happens to the publisher, it goes over to the subscribers and calls the specific notification
  method on their objects.
- It is crucial that all subscribers implement the same interface and that the publisher communicate with them only via
  that interface.
- The interface should declare the notification method, along with a set of parameters that the publisher can use
  to pass some contextual data along with the notification.
- If the app has several different types of publishers, and youw ant to make your subscribers compatible with all of them,
  you can make all publishers follow the same interface.
- The interface would only need to describe a few subscription methods. The interface would allow subscribers to observe
  publisher's states without coupling to their concrete classes.
