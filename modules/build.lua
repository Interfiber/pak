local module = {}
local utils = require "modules.utils"

module.buildProject = function ()
    print("# Prepping for build...")
    utils.prepBuild()
    print("# Building package components...")
    local conf = require "pak_conf"
    print("# Checking conf...")
    utils.checkConf(conf)
    local components = conf.components
    for i, component in pairs(components) do
        if component.name == nil then
            print("# Exception during build!! The component name must be specified!")
            os.exit(1)
        end
        print("# Build: "..component.name)
        print("# Package Payload: "..component.payload)
        utils.requireBuildFile(component.payload)
        if utils.isFolderEmpty(component.payload) then
            print("# Failed to build component: Payload folder is empty!")
            utils.cleanup()
            os.exit(1)
        end
        utils.mkdir(".build_cache/current/"..component.name.."/")
        os.execute("cp -rf "..component.payload.."/* "..".build_cache/current/"..component.name.."/")
        -- check for scripts
        utils.mkdir(".build_cache/current/scripts/"..component.name)
        if component.scripts ~= nil then
            if component.scripts.postinstall ~= nil then
                os.execute("cp "..component.scripts.postinstall.." .build_cache/current/scripts/"..component.name.."/postinstall")
                os.execute("chmod +x".." .build_cache/current/scripts/"..component.name.."/postinstall")
            end
            if component.scripts.preinstall ~= nil then
                os.execute("cp "..component.scripts.preinstall.." .build_cache/current/scripts/"..component.name.."/preinstall")
                os.execute("chmod +x".." .build_cache/current/scripts/"..component.name.."/preinstall")
            end
        end
        os.execute("pkgbuild --scripts .build_cache/current/scripts/"..component.name.." --identifier "..conf.org_id.."."..component.name.." --version "..conf.version.." --root .build_cache/current/"..component.name.."/ --quiet --install-location "..component.install_folder.." .build_cache/current/pkgs/"..conf.org_id.."."..component.name..".pkg")
        print("# Built package component")
    end
    print("# Generate: Distribution")
    local dist = '<?xml version="1.0" encoding="utf-8" ?>'
    dist = dist..'\n<installer-gui-script authoringTool="Pak" minSpecVersion="1.0">'
    -- Handle apperance stuff here
    dist = dist..'\n<title>'..conf.project_name..'</title>'
    if conf.apperance.welcome_html ~= nil then
        utils.requireBuildFile(conf.apperance.welcome_html)
        os.execute("cp "..conf.apperance.welcome_html.." .build_cache/current/resource/welcome.html")
        dist = dist..'\n<welcome file="welcome.html" mime-type="text/html" />'
    end
    if conf.apperance.readme ~= nil then
        utils.requireBuildFile(conf.apperance.readme)
        os.execute("cp "..conf.apperance.readme.." .build_cache/current/resource/README.txt")
        dist = dist..'\n<readme file="README.txt" mime-type="text/plain" />'
    end
    if conf.apperance.license_file ~= nil then
        utils.requireBuildFile(conf.apperance.license_file)
        os.execute("cp "..conf.apperance.license_file.." .build_cache/current/resource/LICENSE.txt")
        dist = dist..'\n<license file="LICENSE.txt" />'
    end
    if conf.apperance.finished_html ~= nil then
        utils.requireBuildFile(conf.apperance.finished_html)
        os.execute("cp "..conf.apperance.finished_html.." .build_cache/current/resource/install_done.html")
        dist = dist..'\n<conclusion file="install_done.html" mime-type="text/html" />'
    end
    -- Generate the choices here
    local pkgFiles = 0
    local ticker = 0
    for _ in pairs(components) do pkgFiles = pkgFiles + 1 end
    dist = dist..'\n<choices-outline>'
    while ticker ~= pkgFiles do
        dist = dist..'\n<line choice="'..ticker..'_install" />'
        ticker = ticker + 1
    end
    ticker = 0
    dist = dist..'\n</choices-outline>'
    -- Write the pkg-ref for each choice
    for _, pkg in pairs(components) do
        local visible = true
        local selected = true
        if pkg.selected == nil then
            utils.warn("Failed to find value selected, using default value.")
        else
            selected = pkg.selected
        end
        if pkg.visible == nil then
            utils.warn("Failed to find value visible, using default value.")
        else
            visible = pkg.visible
        end
        dist = dist..'\n <choice id="'..ticker..'_install" title="'..pkg.name..'" description="" visible="'..tostring(visible)..'" start_selected="'..tostring(selected)..'">\n<pkg-ref id="'..conf.org_id.."."..pkg.name..'"/>\n</choice>'
        ticker = ticker + 1
    end
    -- Write global pkg-ref
    for _, component in pairs(components) do
        dist = dist..'\n<pkg-ref id="'..conf.org_id.."."..component.name..'" version="1.0" auth="Root">'..conf.org_id.."."..component.name..'.pkg</pkg-ref>'
    end
    dist = dist..'\n</installer-gui-script>'
    local distfile = io.open(".build_cache/current/Distribution", "a")
    distfile:write(dist)
    distfile:close()
    os.execute('productbuild --quiet --distribution .build_cache/current/Distribution --resources .build_cache/current/resource --package-path .build_cache/current/pkgs build/'..conf.pkg_name..'.pkg')
    print("# Cleaning up...")
    if os.getenv("PAK_DISABLE_CLEANUP") == nil then
        utils.cleanup(false) 
    end
    print("# Built files into folder: build")
end

return module
