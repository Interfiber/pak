# Creating a New Project


## Create a new project

To create a new project create a new empty folder, then change you're working directory to be that folder. Then you can 
generate a new project using the following command
```bash
pak new
```

## Using ```pak_conf.lua```
!!! note

    This is not a full overview of all of the keys, and values in pak_conf.lua see the project build file section for more info.

The new project generator will create a file called ```pak_conf.lua``` this is where we will be configuring our project
First lets change our project name by editing the value of ```project_name``` to be whatever we want, I'll use ```My Epic Project```.
