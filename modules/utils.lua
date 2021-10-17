local module = {}


local function fileExists(name)
    local ok, exit, signal = os.execute("if [ -e '"..name.."' ]; then\n exit 0\n else\n exit 1; fi");
    if signal == 0 then
        return true
    else
        return false
    end 
end
module.mkdir = function (file)
    os.execute("mkdir "..file)
end
module.requireBuildFile = function (file)
    if fileExists(file) == false then
        print("# Exception during build!! The build requires the following file to exist: "..file)
        module.cleanup()
        os.exit(1)
    end
end
module.isFolderEmpty = function (folder)
    module.requireBuildFile(folder)
    local files = 0
    local output = io.popen("ls -l "..folder, "r")
    for file in output:lines() do
        files = files + 1
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
    module.mkdir(".build_cache/current")
    module.mkdir(".build_cache/current/scripts")
    module.mkdir(".build_cache/current/pkgs")
    module.mkdir(".build_cache/current/resource")
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