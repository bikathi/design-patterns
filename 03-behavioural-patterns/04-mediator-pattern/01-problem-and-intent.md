Iterator is also known as Intermediary or Controller.
It is designed to help reduce chaotic dependencies between objects, thus restricting direct communication between the objects,
forcing them to collaborate only via a mediator object.

# Problem
- Say you have a dialog for creating and editing customer profiles, consisting of various form controls such as text fields,
  textboxes, checkfields, buttons e.t.c
- Some of the elements may interact with others e.g. selecting the 'I have a dog' checkbox may reveal a hidden text field
  for entering the dog's name.
- Similarly, the submit button may have to validate the data entered into all fields before saving the data.
- By having this logic implemented directly inside the code of the form elements, you make these element's classes much harder
  to re-use in other forms of the app.
- E.g., you won't be able to use that checkbox class inside another form because it is coupled to the dog's text field.