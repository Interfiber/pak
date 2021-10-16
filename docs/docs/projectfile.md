# Project File Docs


## org_id [String]
The organization id
```lua
org_id = "dev.interiber.pak"
```

## pkg_name [String]
The Package Name
```lua
pkg_name = "SuperEpicPackage"
```
!!! warning
    This name should not have any spaces in it

## project_name [String]
The name of the current project
```lua
project_name = "Super Epic Project"
```
!!! note
    Unlike ```pkg_name``` this can have spaces in the name.

## version [String]
The current project version using [Semantic Versioning](https://semver.org/)
```lua
version = "0.0.1"
```

## Section: Apperance

## apperance [Table]
Package apperance data
```lua
apperance = {
    welcome_html = "welcome.html"
}
```
!!! note
    The apperance table must have apperance data inside it.

## apperance.finished_html [String]
Relative path to a html document to be shown after the software installation completes
```lua
apperance.finished_html = "complete.html"
```

## apperance.license_file [String]
Relative path to a txt file of the project license, the user must agree to this in order to proceed with the software install
```lua
apperance.license_file = "LICENSE.txt"
```

## apperance.readme [String]
Relative path to a README file, this is shown to use the user before the install.
```lua
apperance.readme = "README.txt"
```
!!! note
    The mime type of the README is ```text/plain``` but if the file contains html, the installer software will set the mime type to ```text/html```

## apperance.welcome_html [String]
Relative path to a html file to be shown to the user on the very first page of the installer

```lua
apperance.welcome_html = "welcome.html"
```

## Section: Components

## components [Table]
List of installer components to build

```lua
components.amazing = {
    -- installer component data here
}
```

!!! note
    For more info on how a installer component data structure is created look at the section below

## Section: Component

## component.install_folder [String]
Direct path to the installation location
```lua
components.amazing.install_folder = "/Library/AmazingSoftware"
```

!!! note
    If the given path does not exist it will create it automatically

## component.name [String]
Name of the installer component
```lua
components.amazing.name = "AmazingSoftware"
```
!!! warning
    This name should not have any spaces in it

## component.payload [String]
Relative path to the installer payload, everthing from the given folder will be copied into the installer location of the current component
```lua
components.amazing.payload = "dist/AmazingSoftware"
```

## component.visible [Bool]
Dictates if the current component is visible in the installer customization screen
```lua
components.amazing.visible = false
```
!!! note
    If you're package has multiple components, it might be a good idea to make sure the default one is not visible to the user can't select it.

## component.selected [Bool]
Dictates if the current component is selected by default in the installer customization screen
```lua
components.other.selected = true
```
