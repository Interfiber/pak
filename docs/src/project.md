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

# Section: Apperance
Apperance settings apply to the entire installer, the apperance config allows you to change background image, license, readme, welcome, and more.
```json
"apperance": {
    "comment": "Apperance config goes inside this json object"
}
```

## $license [Path]
Relative path to the text file used for the software license. During the install the user will need to agree to this license in order to continue

## $welcomeHtml [Path]
Relative path to the html file used for the welcome screen. The rendered html will be displayed to the user when the installer launches

## $conclusionHtml [Path]
Relative path to the html file used for the conclusion screen. The rendered html will be displayed to the user when the install finishes with success

## $readme [Path]
Relative path to the text file used for the readme screen. The contents of the readme file will be showed to the user before the install starts

## $enableBackground [Bool]
Tells pak if it should look for a background config, default is false

# Section: Background config
The background config is a subsection of the apperance section, as the config for it will be placed inside the apperance object likle below:
```json
"apperance": {
    "$backgroundConfig": {}
}
```
Note that pak will not look for a background config unless ```$enableBackground``` is set to true in the apperance config

## $imageFile [Path]
Relative path to the image file used as the background

## $align [String]
Align type for the image, types are:

```center```, ```left```, ```right```, ```top```, ```bottom```, ```topleft```, ```topright```, ```bottomleft```, and ```bottomright```