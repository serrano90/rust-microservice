# Rust Microservices and React App Monorepo

## Getting Starting

This is the small solution example using a Monorepo with **[Rust](https://www.rust-lang.org/)** for the microservice in the backend and the two application in **[ReactJs](https://reactjs.org/)** for the frontend.

## Run
Make sure you have installed and configured docker in your environment. After that, you can run the next command
```sh
    docker-compose build && docker-compose up
```

You should be able to browse different components of the application by using the below URLs :

```sh
    Web BackOffice :  http://localhost:3000/
    Web Customer :  http://localhost:3001/
```
## Structure Folder
The project structured is the next
- **[/api]():** Contain the postman collection.
- **[/configs]():** Contains all file template by configuration.
- **[/deploy]():** Contains the definition for internal resources we need to upload for internal services.
- **[/src]():** Contains all definitions for service and web application.