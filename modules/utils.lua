local module = {}
local lfs = require "lfs"

module.requireBuildFile = function (file)
    if lfs.attributes(file) == nil then
        print("# Exception during build!! The build requires the following file to exist: "..file)
        module.cleanup()
        os.exit(1)
    end
end
module.isFolderEmpty = function (folder)
    module.requireBuildFile(folder)
    local files = 0
    for file in lfs.dir(folder) do
        if file ~= "." and file ~= ".." then
            files = files + 1
        end
    end
    if files == 0 then
        return true
    else
        return false
    end
end
module.cleanup = function (removeBuildFiles)
    os.execute("rm -rf .build_cache/current")
    if removeBuildFiles == true or removeBuildFiles == nil then
        os.execute("rm build/* &>/dev/null") 
    end
end
module.prepBuild = function ()
    os.execute("rm -rf .build_cache/current &>/dev/null")
    module.requireBuildFile("pak_conf.lua")
    module.requireBuildFile("build")
    module.requireBuildFile(".build_cache")
    lfs.mkdir(".build_cache/current")
    lfs.mkdir(".build_cache/current/scripts")
    lfs.mkdir(".build_cache/current/pkgs")
    lfs.mkdir(".build_cache/current/resource")
end
module.warn = function (message)
    print("# Pak WARN  "..message)
end
module.checkConf = function (conf)
    if conf.project_name == nil then
        print("# Exception during build!! The project name must be specified")
        module.cleanup()
        os.exit(1)
    end
    if conf.pkg_name == nil then
        print("# Exception during build!! The package name must be specified")
        module.cleanup()
        os.exit(1)
    end
    if conf.org_id == nil then
        print("# Exception during build!! The org id must be specified")
        module.cleanup()
        os.exit(1)
    end
    if conf.apperance == nil then
        print("# Exception during build!! The apperance config must be specified")
        module.cleanup()
        os.exit(1)
    end
    if conf.components == nil then
        print("# Exception during build!! The components config must be specified")
        module.cleanup()
        os.exit(1)
    end
end
return module