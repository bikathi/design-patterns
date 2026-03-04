- The mediator pattern suggests that you should cease all communication between the components that you want to make
  independent of each other.
- Instead, these components must collaborate indirectly by calling a special mediator object that redirects the calls to
  appropriate components.
- Consequently, the components depend only on a single mediator class instead of being coupled to dozens of their colleagues.
- In our example, the dialog class itself may act as the mediator, receiving notifications from the elements in the form
  then doing the validations, checks.
