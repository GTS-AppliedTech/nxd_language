# METADATA FORMAT (JSON-LD)
***(Template, ID Prefix Key, Example)***


### ***FORMATTING TEMPLATE***


#### JSON-LD
```jsonld
{
  "@context": "https://nxdlang.org/schema",

  "doc_id": "",
  "title": "",
  "description": "",

  "category": "",
  "layer": "",

  "keywords": [],

  "version": "1.4.5",

  "status": "active"
}
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
{
  "@context": "https://nxdlang.org/schema",

  "doc_id": "ME010",
  "title": "Authentication User Application",

  "description": "Demonstrates multi-module authentication architecture.",

  "category": "MultiModule",
  "layer": "medium",

  "keywords": [
    "authentication",
    "security",
    "modules",
    "services"
  ],

  "version": "1.4.5",

  "status": "active"
}
```