local project = require "modules.project"
local build = require "modules.build"
local args = arg

if #args == 0 then
    print("pak - Build MacOS package installers.")
    print("Commands:")
    print("    new      Create a new project in the current folder")
    print("    build      Build the current project")
    os.exit(1)
end
if args[1] == "new" then
    project.createNew()
end
if args[1] == "build" then
    build.buildProject()
end