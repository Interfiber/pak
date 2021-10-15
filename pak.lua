local argparse = require "argparse"
local project = require "modules.project"
local build = require "modules.build"

local parser = argparse("pak", "Easily build MacOS installers")
parser:command("new", "Create a new pak project config in the current folder")
parser:command("build", "Build the current project")
parser:command("clean", "Clear all build caches for the project")

local args = parser:parse()
if args.new then
    project.createNew()
end
if args.build then
    build.buildProject()
end