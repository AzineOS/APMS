# APMS

APMS - Azine Package Management System

The main package manager for Azine based systems. You can find more about Azine at https://github.com/AzineOS/Azine. APMS delievers packages efficiently and is built on rust.

# Have APMS installed?

Use `apms help` for information on how to install, update and remove packages!

# Don't have APMS installed yet?

Here's how to install APMS on your system: <br>

## Setup your toolset
To clone and use APMS development builds, you need to have the following things installed:<br>
1. [Rust](https://www.rust-lang.org/tools/install)
2. [Git](https://git-scm.com/downloads)

## Downloading and compiling
### Downloading

To download APMS from the development server, type the either of the following commands:<br>
`git clone https://github.com/AzineOS/APMS.git -b dev` (download via HTTPS)<br>
OR<br>
`git clone git@github.com:AzineOS/APMS.git` (download via SSH)<br><br>
More information on HTTPS and SSH cloning is available [here](https://stackoverflow.com/questions/11041729/git-clone-with-https-or-ssh-remote)<br><br>

## Compiling
Once you have downloaded APMS, use your terminal to navigate into the project files.<br>
Then you can compile the porject using `cargo build`.

## Running
To run APMS, you can either run the compiled binaries from the `target` output directory, or you can directly use `cargo` to run the project.<br>
<br>
In order to access APMS's network functionality, you need to connect to a server. Don't have one? Don't worry, we provide one for you! Simply run the `test-server` crate included in this package and then start APMS!

## Configuring
APMS lets you configure what server you connect to, to do so, edit the `hosts.txt` file in APMS's configuration directory.<br>
The configuration directory differs between operating systems so here's a quick rundown:<br>
- Windows: `C:\Users\<Your User>\AppData\Roaming\azine\apms\config\`
- MacOS: `/Users/<Your User>/Library/Application Support/com.azine.apms/`
- Linux: `~/.config/apms/`

# How do I use the test server?
Details on how to use the test server can be found in the [server README](test-server/README.md).

# TODO
- [X] Downloading Packages
- [X] Extracting Packages
- [ ] Dependency Resolution
- [ ] Update Packages
- [X] Remove Packages
- [X] Change Package Host URL
- [ ] File Integrity
- [ ] Version Control
- [X] Package Search
- [X] Package Hosting Server
