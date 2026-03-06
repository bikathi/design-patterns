Also known as Event-Subscriber or Listener.
It is a behavioral design pattern that lets you define a subscription mechanism to notify multiple objects about any events
that happen to the object they're observing.

# Problem
- Imagine you have to types of objects, a `Customer` and a `Store`. The customer is very interested in a particular brand
  of product which should be in the store very soon.
- The customer could visit the store every dat and check product availability, but wheile the product is still enroute,
  most of these trips would be pointless.
- On the other hand, the store could send tons of emails (spamming) to all customers each time a new product becomes
  available.
- This would save some customers from endless trips to the store, while upseting others who aren't interested in new
  products.
- So what now? How do we solve the conflict of interest here?
