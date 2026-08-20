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

#### Root (/Docs):
RO001 - RO899 (Project Language Docs)
RO900 - RO999(Project Repo Docs)

####  TESTS 
*all tests retain the same 3 digit portion,*
*of the metadata id, PASS or FAIL.*
PT001 - PT299 (Nim-passed tests)
PT301 - PT599 (D-passed tests)
PT601 - PT999 (Elixir-passed tests)
FT001 - FT299 (Nim-failed tests)
FT301 - FT599 (D-failed tests)
FT601 - FT999 (Elixir-failed tests)
CT001 (Compiler tests)

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