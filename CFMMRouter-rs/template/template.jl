using PkgTemplates

lib_template = Template(
    dir="..",
    plugins=[
        PackageCompilerLib(lib_name="cfmmrouter")
    ]
)
lib_template("CFMMROUTER")
