# Getting Started
How to get started with pak

## Installation
Theres a [Installation Guide](/install) for this.

## Creating a new project
First create a new empty folder, and run the following command inside it
```bash
pak new
```
This will bootstrap a new project for us. Now we can move onto making our package

## Project file overview
Lets start with adding a basic installer component to our project file, in the components table there should be a precreated one for us. Lets look at what this does. First we give the component a ```name```, this tells us what component it is. Next we set the ```install_folder```, this tells us where we should install the software. Then we give it a ```payload``` which is a folder which the contents of will be installed into the install folder. Then we tell the installer if this component is ```visible``` in the installer customize window. Then finally we tell the installer if it should start ```selected``` in the customize window.

## Building the package
To build the package we can run the following command in our project folder:
```bash
pak build
```
This will create a package installer in the folder ```build```