local serpent = require "modules.serpent"
local lfs = require "lfs"
local module = {}

module.createNew = function ()
    print("# Creating new project config file...")
    local projectData = serpent.block({
        project_name = "My Cool Software",
        version = "0.0.1",
        pkg_name = "MyCoolSoftware",
        org_id = "com.company.product",
        apperance = {
            welcome_html = "epic.html",
            finished_html = "super_epic.html",
            readme = "README.txt",
            license_file = "LICENSE.txt",
            background_png = "background.png"
        },
        scripts = {
            preinstall = "scripts/preinstaller",
            postinstall = "script/postinstaller"
        },
        components = {
            default = {
                name = "Default",
                visible = true,
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
        lfs.mkdir(folder)
        print("# Created folder: "..folder)
    end
    print("# Created new project.")
end
return module