local module = {}
local lfs = require "lfs"

module.buildProject = function ()
    print("# Building project from config file")
    if lfs.attributes("pak_conf.lua") == nil then
        print("# Failed to build project: Build config does not exist")
        os.exit(1)
    end
    if lfs.attributes("build") == nil then
        print("# Failed to build project: Build folder does not exist")
        os.exit(1)
    end
    if lfs.attributes(".build_cache") == nil then
        print("# Failed to build project: .build_cache folder does not exist")
        os.exit(1)
    end
    local conf = require "pak_conf"
    print("# Prepping for build...")
    lfs.mkdir(".build_cache/current")
    lfs.mkdir(".build_cache/current/scripts")
    lfs.mkdir(".build_cache/current/pkgs")
    lfs.mkdir(".build_cache/current/result")
    lfs.mkdir(".build_cache/current/resource")
    print("# Building package components...")
    local components = conf.components
    for i, component in pairs(components) do
        print("# Build: "..component.name)
        lfs.mkdir(".build_cache/current/"..component.name.."/")
        os.execute("cp -rf "..component.payload.."/* "..".build_cache/current/"..component.name.."/")
        os.execute("pkgbuild --identifier "..conf.org_id.."."..component.name.." --version "..conf.version.." --root .build_cache/current/"..component.name.."/ --install-location "..component.install_folder.." .build_cache/current/pkgs/"..conf.org_id.."."..component.name..".pkg")
        print("# Built package component")
    end
    print("# Generate: Distribution")
    local dist = '<?xml version="1.0" encoding="utf-8" ?>'
    dist = dist..'\n<installer-gui-script authoringTool="Pak" minSpecVersion="1.0">'
    -- Handle apperance stuff here
    dist = dist..'\n<title>'..conf.project_name..'</title>'
    if conf.apperance.welcome_html ~= nil then
        os.execute("cp "..conf.apperance.welcome_html.." .build_cache/current/resource/welcome.html")
        dist = dist..'\n<welcome file="welcome.html" mime-type="text/html" />'
    end
    if conf.apperance.readme ~= nil then
        os.execute("cp "..conf.apperance.readme.." .build_cache/current/resource/README.txt")
        dist = dist..'\n<readme file="README.txt" mime-type="text/plain" />'
    end
    if conf.apperance.license_file ~= nil then
        os.execute("cp "..conf.apperance.license_file.." .build_cache/current/resource/LICENSE.txt")
        dist = dist..'\n<license file="LICENSE.txt" />'
    end
    if conf.apperance.finished_html ~= nil then
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
        dist = dist..'\n <choice id="'..ticker..'_install" title="'..pkg.name..'" description="" visible="'..tostring(pkg.visible)..'" start_selected="'..tostring(pkg.selected)..'">\n<pkg-ref id="'..conf.org_id.."."..pkg.name..'"/>\n</choice>'
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
    os.execute('productbuild --distribution .build_cache/current/Distribution --resources .build_cache/current/resource --package-path .build_cache/current/pkgs build/'..conf.pkg_name..'.pkg')
    print("# Cleaning up...")
    os.execute("rm -rvf .build_cache/current")
    print("# Built files into folder: build")
end

return module