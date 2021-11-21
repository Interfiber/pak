# Installer Scripts
Installer scripts are files that contain code to be execute during the install process


## Adding scripts to a component
To declare a folder to load scripts from we need to set ```$scriptsFolder``` in the component we want to add scripts to like below:
```json
"$scriptsFolder": "scripts"
```
This will tell pak to build the component using any scripts from the ```scripts``` folder.

## Preinstall scripts
To add a preinstall script, create a new file in the folder storing you're scripts called ```preinstall``` this should have no file extension
Then make the file executable like this:
```bash
chmod +x preinstall
```
The code inside preinstall by default uses shell script, but it can be changed by adding a [shebang](https://bash.cyberciti.biz/guide/Shebang)

## Postinstall scripts
To add a postinstall script, create a new file in the folder storing you're scripts called ```postinstall``` this should have no file extension
Then make the file executable like this:
```bash
chmod +x postinstall
```
The code inside postinstall by default uses shell script, but it can be changed by adding a [shebang](https://bash.cyberciti.biz/guide/Shebang)