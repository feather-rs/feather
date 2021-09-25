# Components and Queries in Quill

There are two types of components accessed by a plugin:
* "Plain-old-data" components, typically implementing `Copy`. To transfer these to a plugin,
the raw struct bytes are copied into the plugin's memory, and the plugin gets a pointer to that data.
* Opaque components, i.e., those not implementing `Copy`. These typically hold more data. Examples: `Inventory` of , `Window`.
A plugin accesses these components via host calls without ever getting a copy of the component itself. For example,
to get an item from an inventory, a plugin calls `quill_entity_get_inventory_item`. (The high-level `quill` API wraps
this raw call with an `Inventory` struct, but the struct doesn't actually hold the inventory. It's just a marker.)
