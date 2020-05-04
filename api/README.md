`fapi`, the plugin API used for Feather plugins.

# Terminology

"Host" refers to the server executable itself, while "plugin" refers to the plugin dynamic library.

# Contents

`common`: types and functions shared between the host and plugin implementation

`macros`: procedural macros exposed to plugins

`.`: the high-level plugin API exposed to plugins themselves

`test-plugins`: test plugin crates