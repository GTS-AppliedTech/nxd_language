---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "ES002",
  "title": "",
  "description": "",
  "layer": "ecosystem",
  "category": "ecosystem",
  "keywords": [],
  "doc_version": "",
  "status": "active"
}
---


# ES002 Module & Package System Specification

NXD’s module system defines how code is organized, named, imported, exported, versioned, and distributed across projects and backends.
It must unify three incompatible ecosystems:

• Nim modules
• Elixir modules
• D modules/packages


The user sees one coherent system.
Backends map it to their native systems.



1. Module definition

A module is the smallest unit of code organization in NXD.

# Syntax

MODULE math.geometry


# Semantics

• Defines a namespace.
• Corresponds to a single .nxd file.
• Module names use dot‑notation for hierarchy.
• The module name must match the file path.


# File mapping

MODULE math.geometry


→ file path:

math/geometry.nxd


# Backend mapping

• Nim → math/geometry.nim
• Elixir → math/geometry.ex with defmodule Math.Geometry
• D → math/geometry.d with module math.geometry;




2. Import system

NXD uses a unified import syntax:

IMPORT math.geometry
IMPORT math.geometry AS geo
IMPORT security.crypto HASH


# Forms

2.1. Simple import


IMPORT math.geometry


2.2. Aliased import


IMPORT math.geometry AS geo


2.3. Selective import


IMPORT security.crypto HASH


# Semantics

• Imports are lexically scoped to the module.
• Aliases create local namespace shortcuts.
• Selective imports bring specific symbols into scope.


# Backend mapping

Nim

import math/geometry
import math/geometry as geo
from security/crypto import hash


Elixir

alias Math.Geometry
alias Math.Geometry, as: Geo
import Security.Crypto, only: [hash: 1]


D

import math.geometry;
import math.geometry : geo;
import security.crypto : hash;



3. Export system

NXD uses explicit exports:

EXPORT FUNC AREA
EXPORT TYPE POINT
EXPORT ALL


# Rules

• EXPORT ALL exports all public declarations.
• Individual exports allow fine‑grained control.
• Unexported symbols remain module‑private.


# Backend mapping

Nim

proc area*() = ...
type Point* = object ...


Elixir

def area(...), do: ...
defstruct [:x, :y]   # structs are public by default


D

public double area(...) { ... }
public struct Point { ... }



4. Package system

NXD packages are collections of modules with metadata.

# Package file

Every package contains a package.nxd manifest:

name = "geometry"
version = "1.2.0"
license = "MIT"

[dependencies]
math = ">=1.0.0"
security.crypto = "^2.1.0"


# Semantics

• Defines package identity.
• Defines versioning rules.
• Defines dependency constraints.
• Used by the NXD build system and package manager.


# Backend mapping

NXD packages map to:

• Nim → Nimble packages
• Elixir → Mix packages
• D → Dub packages


The NXD compiler generates backend manifests automatically.



5. Visibility rules

NXD defines three visibility levels:

• public — exported symbols
• private — module‑local symbols
• internal — visible within the same package


# Syntax

PUBLIC FUNC AREA
PRIVATE FUNC NORMALIZE
INTERNAL TYPE VECTOR


# Backend mapping

• Nim → * for public, no marker for private
• Elixir → public functions by default, private via defp
• D → public, private, package



6. Module initialization

NXD modules may define initialization blocks:

INIT:
    PRINTLN("geometry module loaded")


# Backend mapping:

• Nim → static: blocks
• Elixir → module attributes or @on_load
• D → static this()



7. Cross‑backend consistency rules

To ensure portability:

• Module names must be lowercase with dots.
• No backend‑specific naming conventions allowed.
• No backend‑specific import paths allowed.
• No backend‑specific visibility keywords allowed.
• No backend‑specific initialization semantics allowed.


NXD enforces these rules at compile time.



8. Example module

NXD

MODULE math.geometry

EXPORT TYPE POINT
EXPORT FUNC AREA

TYPE POINT { X: float, Y: float }

FUNC AREA(P):
    RETURN P.X MUL P.Y


Nim

# math/geometry.nim
type Point* = object
  x: float
  y: float

proc area*(p: Point): float =
  p.x * p.y


Elixir

# math/geometry.ex
defmodule Math.Geometry do
  defstruct [:x, :y]

  def area(%Math.Geometry{x: x, y: y}) do
    x * y
  end
end


D

// math/geometry.d
module math.geometry;

public struct Point {
    double x;
    double y;
}

public double area(Point p) {
    return p.x * p.y;
}



## Summary

The NXD module & package system is:

• Unified across Nim, Elixir, and D
• Semantic, not backend‑specific
• Explicit, with clear imports/exports
• Hierarchical, using dot‑notation
• Portable, with backend manifests generated automatically
• Secure, with visibility and initialization rules
• Scalable, supporting large multi‑module projects


This system is the backbone of NXD’s project structure and build pipeline.

