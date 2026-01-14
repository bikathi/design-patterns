# Intent
Is a structural design pattern that allows objects with incompatible interfaces to collaborate.

# Probem
- You are creating a stock market monitoring app.
- Your app downloads data from multiple sources in XML format and then displays charts and diagrams for the user.
- You then decide to improve the app by integrating a smard 3rd party analytics library.
- But here is the catch: the library only works with data in JSON format.
- You don't have access to the library's code to make it work with XML, so what now?