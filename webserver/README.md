# Rust Webserver
This is the Rust backend of the project

## Stack
- Axum as webframework
- Diesel as ORM

### Structure
- **User Account**
  - Library ID
  - name, etc
- **Library**
  - id
  - composers: [uuids]
  - scores: [uuids]
- **Score**
  - composer: uuid
  - filepath
  - ...
