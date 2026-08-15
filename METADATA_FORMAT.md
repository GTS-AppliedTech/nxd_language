---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "RO903",
  "title": "",
  "description": "",
  "layer": "root",
  "category": "metadata",
  "keywords": [],
  "doc_version": "1.0",
  "status": "active"
}
---


# RO903 METADATA FORMAT 
***(Template, ID Prefix Key, Example)***


### ***FORMATTING TEMPLATE***


#### JSON-LD
```jsonld
---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "",
  "title": "",
  "description": "",
  "layer": "",
  "category": "",
  "keywords": [],
  "doc_version": "",
  "status": "active"
}
---
```


### BASE METADATA ID PREFIX KEY
***(By chapter, sub-chapter, and root)***


#### Examples:
Tiny - TE001
Medium - ME001
Large - LE001
XLarge - XE001

#### Reference Architechture:
RA001

#### Cookbook Appendix:
CB001

#### Backend:
BE001

#### Compiler:
CP001

#### Ecosystem:
ES001

#### Language Guide:
LG001

#### Runtime:
RT001

#### Security:
SE001

#### Root (/Docs):
RO001


### ***FORMATTING EXAMPLE***


#### JSON-LD
```jsonld
---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "ME010",
  "title": "Authentication User Application",
  "description": "Demonstrates multi-module authentication architecture.",
  "layer": "Medium",
  "category": "MultiModule",
  "keywords": [
    "authentication", 
    "security",
    "modules", 
    "services",
   ],
  "doc_version": "1.0",
  "status": "active"
}
---
```