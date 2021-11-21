# Project File Docs
Docs for the ```pak.project.json``` file in every pak project

## projectName [String]
The name of the project, also displayed as the title in the installer
```json
"projectName": "Epic Project"
```

## version [String]
Current package version
```json
"version": "0.0.1"
```

## orgName [String]
The organization id for the current project
```json
"orgName": "dev.interfiber.pak"
```` 

## components [Array]
Array of components to build into the final package
```json
"components": [
    "default"
]
```

# Section: Components
Components are declared as objects in the project file like this:
```json
"component_super_epic": {

}
```
the name of the object must start with ```component``` then a underscore and a name with no spaces
To compile the component into the package the name must be listed in the ```components``` array.

## $name [String]
Name of the component displayed to the user, this can contain spaces.

## $pkgName [String]
Name of the component **WITHOUT SPACES**, used to generate the raw packages

## $installDir [Path]
Path to the installation directory

## $payloadName [String]
Name of the folder containing the payload located in the ```payloads``` folder.

## $selectable [Bool]
If the component should be selectable when the user is choosing what packages to be installed

## $selected [Bool]
If the component should start selected when the user is choosing what packages to be installed

## $visible [Bool]
If the component should be visible to the user when choosing what packages to be installed

## $scriptsFolder [Path]
Relative path to the folder containing scripts for the package
For more info on scripts see the [Scripts](scripts.md) section