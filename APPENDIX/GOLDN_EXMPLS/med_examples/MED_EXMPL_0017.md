# RUNTIME + INIT + CYCLIC-SAFE IMPORTS

### JSON-LD Metadata
```jsonld
{
  "@context": "https://schema.nxd.dev/example",
  "example_id": "nxd_medium_07_runtime_init_graph",
  "category": "Runtime",
  "layer": "medium",
  "features": ["init", "imports", "topological-order"]
}
```


### NXD
```nxd
MODULE runtime.a
IMPORT runtime.b

INIT:
    PRINTLN("init A")

FUNC A():
    PRINTLN("A called")


MODULE runtime.b

INIT:
    PRINTLN("init B")

FUNC B():
    PRINTLN("B called")


MODULE runtime.main
IMPORT runtime.a
IMPORT runtime.b

INIT:
    PRINTLN("init MAIN")

FUNC MAIN():
    A()
    B()
    RETURN NONE
```