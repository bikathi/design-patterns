# Intent
Is a structural desgin pattern that lets you attach new behavious to objects, by placing these objects inside special
wrapper objects that contain the behaviours.

# Problem
- Imagine you're working on a notification library that let's other programs notify their users about special events.
- Your V1 code was based on a single `Notification` class that had only a few fields, a constructor a a single `send()` method.
- The `send()` method accepted a message argument from a client and sent the message to a list of emails that were passed to
  the notifier via its constructor.
- A third party app which acted as a client was supposed to create and configure the notifier object once, and then use it
  each time something important happened.
- You later realize that users of the library expect more than just email notifications. Others expect Facebook, Slack and
  SMS notifications.
- You decide to extend the `Notifier` class e.g `FacebookNotifier`, `SlackNotifier`, `SMSNotifier` e.t.c and put 
  the additional notification methods into new subclasses.
- Now the client is supposed to instantiate the desired notification class and use it for all further notifications.
- But how would they use several notification types at once? 
- You decide to combine several notification methods within one class e.g `SMS+Slack Notifier`, 
  `SMS+Facebook+Slack Notifier` e.t.c, but quickly, this becodes bloated and difficult to use.
- We need to find another way to structure the notification classes.