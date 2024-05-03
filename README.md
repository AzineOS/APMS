# APMS

APMS - Azine Package Management System

The main package manager for Azine based systems. You can find more about Azine at https://github.com/AzineOS/Azine. APMS delievers packages efficiently and is built on rust.

# Have APMS installed?

Use `apms help` for information on how to install, update and remove packages!

# Don't have APMS installed yet?

Open a terminal and type `git clone https://github.com/AzineOS/APMS -b dev`<br>
`cd APMS` then `cd apms` (2 SEPERATE COMMANDS, DO THEM BOTH.)<br>
Ensure you have RUST installed on your system.<br>
Type `cargo build` and wait.<br>

Then it's done! That's APMS installed. To use APMS at current you need to use the test server, see below for more information.

# How do I use the test server?
Details on how to use the test server can be found in the [server README](test-server/README.md).

# TODO
- [X] Downloading Packages
- [ ] Extracting Packages
- [ ] Dependency Resolution
- [ ] Extracting Packages
- [ ] Update Packages
- [ ] Remove Packages
- [X] Change Package Host URL
- [ ] File Integrity
- [ ] Version Control
- [ ] Package Search
- [ ] Package Hosting Server
