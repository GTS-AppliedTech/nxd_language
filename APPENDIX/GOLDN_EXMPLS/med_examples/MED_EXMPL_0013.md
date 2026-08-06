# OWNERSHIP WITH COLLECTIONS (MOVE, BORROW, CLONE ON LIST)

### JSON-LD Metadata
```jsonld
{
  "@context": "https://schema.nxd.dev/example",
  "example_id": "nxd_medium_03_ownership_collections",
  "category": "Ownership",
  "layer": "medium",
  "features": ["move", "borrow", "clone", "lists", "mutation"]
}
```


## NXD
```nxd
MODULE ownership.collections

FUNC MAIN():
    LET L SET [1, 2, 3]

    LET B SET BORROW L
    PRINTLN(B[0])

    LET C SET CLONE L
    C[0] SET 99

    LET D SET MOVE L

    PRINTLN(D[0])
    PRINTLN(D[1])
    PRINTLN(D[2])

    RETURN NONE
```