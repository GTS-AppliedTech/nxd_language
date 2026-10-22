{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "TS004",
  "title": "TS004 Test Numbering Standard",
  "description": "Reserved ID numbers for testing",
  "layer": "Test Suite",
  "category": "",
  "keywords": [],
  "doc_version": "1.0",
  "status": "active"
}


# TS004 - NXD Test Numbering Standard
Purpose

This document defines the official numbering methodology used by the NXD Test Suite.

The numbering standard provides:

Unique test identification
Backend separation
Positive/negative categorization
Artifact traceability
Future growth capacity
Auditability and historical consistency

All NXD tests MUST be assigned a unique test number in accordance with this standard.

### Design Philosophy

NXD test numbers are intentionally structured to provide meaningful information.

A test number identifies:

Backend Target
Test Category
Test Purpose

Example:

001


Immediately indicates:

Nim Backend
Positive Validation Test


Example:

742


Immediately indicates:

Elixir Backend
Negative Validation Test


No additional lookup tables should be required to determine the category of a test.

### Backend Allocation

Test numbers are allocated by backend.

Backend	Range

Nim	001-299
D	301-599
Elixir	601-899
Reserved	901-999

Test Category Allocation

Each backend range is divided into three sections.

#### Positive Validation Tests

Purpose:

Valid NXD programs expected to compile successfully.


Ranges:

Backend	Range

Nim	001-099
D	301-399
Elixir	601-699

#### Negative Validation Tests

Purpose:

Invalid NXD programs expected to fail compilation.


Ranges:

Backend	Range

Nim	101-199
D	401-499
Elixir	701-799

#### Backend-Specific Specialized Tests

Purpose:

Backend-specific behavior
Runtime integration
Backend feature validation
Backend limitations
Backend capability testing


Ranges:

Backend	Range

Nim	201-299
D	501-599
Elixir	801-899

#### Reserved Range

The following block is currently reserved:

901-999


This range may be assigned to future testing categories including but not limited to:

Runtime Validation
Cross-Backend Conformance
Experimental Features
Performance Testing
Integration Testing
Future Backend Targets

Assignment of this range shall be determined by future project requirements.

### Test Equivalency Model

Many NXD tests verify identical language behavior across multiple backends.

To maintain output traceability, each backend receives a unique test identifier.

Example:

###### Feature            |	Nim	 |  D  | Elixir|
------------------------------------------------
###### Basic Print        |	001	 | 301 |  601  |
###### Variable Assignment|	002	 | 302 |  602  |
###### Function Call      |	003  | 303 |  603  |

Although the NXD source may be identical, each backend test receives an independent identifier.

Backend Artifact Traceability

Separate identifiers prevent ambiguity when reviewing generated artifacts.

Example:

001_nim_output.nim

301_d_output.d

601_elixir_output.ex


Reviewers can immediately identify:

Source backend
Test category
Test identifier

without opening the file.

### Positive Test Numbering Rules

Positive tests SHALL:

Contain valid NXD programs.
Successfully complete the intended validation pipeline.
Occupy only the positive validation ranges.

Example:

*Nim* 001-099
*D* 301-399
*Elixir* 601-699


Positive tests MUST NOT be assigned negative test identifiers.

### Negative Test Numbering Rules

Negative tests SHALL:

Contain intentionally invalid NXD programs.
Be expected to fail compilation.
Occupy only the negative validation ranges.

Example:

*Nim* 101-199
*D* 401-499
*Elixir* 701-799


Negative tests MUST NOT be assigned positive test identifiers.

### Specialized Test Numbering Rules

Specialized tests validate backend-specific functionality.

Examples may include:

Nim:
ARC
ORC
Compile-time execution
Template interaction
Macro interaction

D:
@safe validation
@nogc validation
Runtime integration
Template behavior
Range integration

Elixir:
Process spawning
Message passing
OTP interaction
Supervisor behavior
BEAM-specific capabilities

These tests occupy the backend-specific ranges:

*Nim* 201-299
*D* 501-599
*Elixir* 801-899

### Test Identifier Lifecycle

Test identifiers are permanent records.

Once assigned, a test identifier SHOULD NOT be reused.

Status examples:

Allocated
Implemented
Validated
Deprecated
Retired


Retired tests retain their original identifiers for historical traceability.

### Number Reservation Practices

When practical, numbers SHOULD be assigned sequentially.

Example:

001
002
003
004


Gaps MAY be intentionally left for future expansion.

Example:

001
002
005
006


Reserved numbers should be documented where applicable.

### Future Expansion

The numbering system is intentionally designed to support:

Additional backend-specific testing
Runtime validation testing
Future compiler targets
Future execution environments
Long-term project growth

The numbering standard is expected to remain stable without requiring large-scale renumbering of existing tests.

### Summary

The NXD Test Numbering Standard provides:

Backend separation
Positive and negative categorization
Specialized testing allocation
Artifact traceability
Auditability
Future scalability

Every test identifier serves as a permanent, uniquely identifiable validation record within the NXD Test Suite.