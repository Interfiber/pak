local serpent = require "modules.serpent"
local utils = require "modules.utils"
local module = {}

module.createNew = function ()
    print("# Creating new project config file...")
    local projectData = serpent.block({
        project_name = "My Cool Software",
        version = "0.0.1",
        pkg_name = "MyCoolSoftware",
        org_id = "com.company.product",
        apperance = {
            comment = "Apperance config goes here. You can delete this comment line if you want"
        },
        components = {
            default = {
                name = "Default",
                visible = true,
                scripts = {
                    comment = "script data for preinstall, and postinstall scripts goes here. You can delete this comment line if you want"
                },
                start_selected = true,
                payload = "payloads/default",
                install_folder = "/Users/Shared/DefaultInstall",
            }
        }
    })
    local projectFile = io.open("pak_conf.lua", "a")
    projectFile:write("return "..projectData)
    projectFile:close()
    print("# Created new project config file.")
    print("# Creating project folders")
    local folders = {
        "payloads",
        "build",
        ".build_cache",
        "payload/default"
    }
    for i, folder in pairs(folders) do
        utils.mkdir(folder)
        print("# Created folder: "..folder)
    end
    print("# Created new project.")
end
return module
