# Plugins

## Install
### To install Feather, if you are running Unix,
run the following in your terminal, then follow the onscreen instructions.
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.feathermc.org | sh
```

### If you are running Windows 64-bit,
download and run \
[feather-init.exe](https://win.feathermc.org/x86_64) \
then follow the onscreen instructions.

### If you are running Windows 32-bit,
download and run \
[feather-init.exe](https://win.feathermc.org/i686) \
then follow the onscreen instructions. 


## Commands
### Init
To create a server in an existing directory
```sh
feather init [path]
```
This command will create a new feather server in the current directory.
Give a path as an argument to create in the given directory.

### Start
To start a server in an existing directory
```sh
feather start [path]
```
This command will run the feather server in the current directory.
Give a path as an argument to run in the given directory.

### Add plugin
```sh
feather add <plugin-identifier> 
```

### Remove plugin
```sh
feather remove <plugin-identifier> 
```

### Upgrade
To upgrade plugins in an exsisting directory
```sh
feather upgrade [path]
```
This command will upgrade the feather server in the current directory.
Give a path as an argument to run in the given directory.

To upgrade the feather-cli 
```sh
feather upgrade --self
```
This command will upgrade the feather-cli 
