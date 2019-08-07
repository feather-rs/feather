See block_format.md for more information on data types.

* Header
    * Raw bytes corresponding to "FEATHER_ITEM_DATA_FILE" in ASCII

* Protocol ID mappings
    * Array of struct
        * `if native_mappings:`
            * String name of this item, e.g. "minecraft:iron_sword"
            * Protocol ID (`i32`)
        * `else:`
            * Native ID for this item (`i32`)
            * Protocol ID of this item for this version (`i32`)