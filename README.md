# Apache NiFi - Rust bindings

Ongoing library that aims to provide abstractions in Rust for using and integrating Apache NiFi.

The main idea is to create a composable and dynamic system to declare NiFi flows and deploy them, without being concerned about
stopping all, replaying all, working with revisions, plugging environment variables. All this is going to be declared and ready to go. 
It's like React for NiFi declarations.

## Roadmap
Currently we're focusing on this aspects: 
* Access and API authorisation
* Parameter Context and Parameter Providers from env files.
* Database Controller Services
* Simple flows, and process groups
* Simple processors like HTTP and kafka stuff

## Specifications
All types and DTOS are taken, and compiled into library by openapi-codegen project. The resulting rust artifacts are 
in `OUT_DIR`. Some patches has been applied, please refer to `build.rs` for more details. 

## Contribute
For contributing please open an Issue, create whatever you need and don't forget to create integration and unit tests. 
That is a library, code without test won't be merged in. 