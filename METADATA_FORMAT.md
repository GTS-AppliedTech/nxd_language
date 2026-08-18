---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "RO930",
  "title": "Metadata Format",
  "description": "Format",
  "layer": "root",
  "category": "metadata",
  "keywords": [example, metadata, format],
  "doc_version": "1.0",
  "status": "active"
}
---


# RO940 METADATA FORMAT 
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

#### SEMANTIC CONFORMITY
SC001

#### SAMPLE TESTS
ST001 - ST299 (Nim)
ST301 - ST599 (D)
ST601 - ST999 (Elixir)

#### Root (/Docs):
RO001 - RO899 (Project Language Docs)
RO900 - RO999(Project Repo Docs)

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