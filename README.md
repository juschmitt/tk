# TickTick CLI

A command-line interface tool for managing tasks and lists using the TickTick API. 

## Features

- **Lists**: List, view, create and delete projects.
- **Tasks**: List, view, create, edit, and delete tasks within projects.

## Installation
To install the CLI tool, clone the repository and install the project using Cargo:

```sh
git clone git@github.com:juschmitt/tk.git
cd tk
cargo install
```

## Usage
### Obtain Client ID and Client Secret
Before you can use the CLI tool, you need to authenticate using your TickTick account. To do this, you need to create a TickTick developer account and register a new application to obtain a client ID and client secret.
Registration can be done here: [TickTick Developer](https://developer.ticktick.com/manage)

### Authenticate
Run the CLI tool using the following command:

```sh
tk auth login --id <client-id> --secret <client-secret>
```
### Create a new Task
To create a new task, use the following command:

```sh
tk task new --name "Buy milk"
```
### GetH Help
Enter `tk` without any command to see the help. 

## Configuration
For editing tasks the $EDITOR environment variable is used. 
To change the editor, set the variable to the desired editor.
