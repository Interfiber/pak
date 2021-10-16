return {
  apperance = {
    background_png = "background.png",
    finished_html = "done.html",
    license_file = "../LICENSE",
    readme = "README.txt",
    welcome_html = "welcome.html"
  },
  components = {
    default = {
      install_folder = "/Users/Shared/DefaultInstall",
      name = "Default",
      payload = "payloads/default",
      visible = false,
      selected = true
    },
    other = {
      install_folder = "/Users/Shared/OtherInstallData",
      name = "Other",
      payload = "payloads/other",
      visible = true,
      selected = false
    }
  },
  org_id = "com.company.product",
  pkg_name = "MyCoolSoftware",
  project_name = "My Cool Software",
  scripts = {
    postinstall = "script/postinstaller",
    preinstall = "scripts/preinstaller"
  },
  version = "0.0.1"
}