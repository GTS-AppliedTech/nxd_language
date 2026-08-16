---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "RO910",
  "title": "",
  "description": "",
  "layer": "Root",
  "category": "Contributing",
  "keywords": [],
  "doc_version": "1.0",
  "status": "active"
}
---


# RO910 CONTRIBUTING.md

## **Welcome**
Thank you for your interest in contributing to **NXD** — a multi‑backend systems language targeting Nim, Elixir, and D.  
This project is currently under active development, and contributions are welcome once the documentation reaches its initial stable baseline.

This guide explains how to contribute effectively and consistently.

---

## **1. Code of Conduct**
All contributors must follow the project’s Code of Conduct (to be added).  
Be respectful, constructive, and collaborative.

---

## **2. Repository Structure**
NXD uses a clear separation between specification, examples, and backend adapters:

```
/
  docs/                 # Language specification (Markdown)
    core/               # Core semantics
    appendix/           # Patterns, backend mapping, examples
    proposals/          # Future extensions (optional)
  examples/             # NXD example programs
  backends/             # Backend lowering notes
    nim/
    elixir/
    d/
  tools/                # Compiler, parser, or utilities (future)
```

When contributing, please place files in the correct directory.

---

## **3. How to Contribute**

### **3.1 Documentation Contributions**
NXD is documentation‑first.  
You may contribute by:

- improving clarity  
- fixing typos  
- adding examples  
- expanding appendices  
- proposing new semantic rules  
- adding backend lowering notes  

All documentation must be written in **Markdown** and follow the existing structure.

### **3.2 Specification Changes**
Changes to the core language spec must:

1. Be discussed in an Issue first.  
2. Include a clear rationale.  
3. Include examples.  
4. Include backend impact notes (Nim, Elixir, D).  
5. Include any required diagnostic codes.

Large changes should be submitted as **proposals** under:

```
docs/proposals/
```

### **3.3 Examples**
Example programs should be placed under:

```
examples/
```

Examples must:

- compile under the reference compiler (when available)  
- demonstrate a specific feature  
- be minimal and readable  

---

## **4. Style Guidelines**

### **4.1 Markdown**
- Use `#` headers consistently  
- Keep lines under ~100 characters  
- Use fenced code blocks with language tags  
- Prefer lists over long paragraphs  
- Avoid trailing whitespace  

### **4.2 NXD Code**
- Use uppercase keywords (`LET`, `FUNC`, `TYPE`, `MATCH`)  
- Prefer explicit types  
- Prefer explicit conversions (`AS`)  
- Use traits for behavior, not inheritance  
- Avoid backend‑specific constructs in core examples  

### **4.3 Commit Messages**
Use descriptive commit messages:

```
Add channel lifecycle examples
Fix typo in capability semantics
Expand dynamic trait object appendix
```

---

## **5. Issues and Discussions**

### **5.1 Filing an Issue**
When filing an issue, include:

- What section of the spec it relates to  
- Expected behavior  
- Actual behavior  
- Backend implications  
- Proposed fix (if any)

### **5.2 Proposals**
Large changes should follow the proposal template:

```
Title
Summary
Motivation
Specification
Examples
Backend Mapping
Diagnostics
Alternatives
```

---

## **6. Pull Requests**

### **6.1 Requirements**
All PRs must:

- Reference an Issue  
- Include a clear description  
- Pass formatting checks (when tooling exists)  
- Include examples if modifying semantics  

### **6.2 Review Process**
PRs are reviewed for:

- correctness  
- clarity  
- consistency  
- portability  
- impact on backends  
- impact on diagnostics  

---

## **7. Backend Notes**
NXD targets three backends:

- Nim  
- Elixir  
- D  

Any change to core semantics must include a short note on how it affects backend lowering.

---

## **8. Licensing**
All contributions are licensed under the **MIT License**, matching the project’s license.

---

## **9. Getting Started**
If you’re new to the project, good first contributions include:

- fixing typos  
- improving examples  
- adding backend lowering notes  
- adding diagnostic codes  
- expanding appendices  

---

## **10. Contact**
For questions, open an Issue or Discussion in the repository.
