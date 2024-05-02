# APMS - Test Server
This is the server we use to test APMS functionality. <br>
WARNING: This server is **not** made to be used in *any* production environment. Only host it on localhost in a development environment!

## Endpoints

- `/packages/<name>/download` - GET, downloads a package
- `/packages/<name>/version` - GET, gets a package's version

## How do I host a package?
When starting the program, it creates a directory called `packages` which hosts all the packages. <br>
Each package is stored in a sub-folder of `packages`, where the `package.zip` and `version.txt` files are located.

### Example Package Structure
This example shows the structure of two example packages called "ATP" (Azine Test Package) and "test"
```text
- packages
    - ATP
        - package.zip
        - version.txt
    - test
        - package.zip
        - version.txt
```