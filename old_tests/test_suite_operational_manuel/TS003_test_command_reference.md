{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "TS003",
  "title": "TS003 Test Command Reference",
  "description": "Test pipeline commands",
  "layer": "Test Suite",
  "category": "",
  "keywords": ["scripts", "commands", "tests", "reference", "Bash", "PowerShell"],
  "doc_version": "2.0",
  "status": "active"
}

# TS003 -  Test Command Reference

### Purpose

The NXD Test Command Reference provides the executable commands used to operate the NXD test suite.

This document focuses exclusively on test execution.

For testing methodology see TS002.

**Linux Note:**
***When running test commands, some Linux Distributions may not provide a python alias by default. If you receive: 'python: command not found' substitute:***

```shell
python3 <test_script_name>.py
```
***In place of:***

```shell
python <test_script_name>.py
```

#### Level 1 Commands
##### Frontend Validation
**Bash**
```Shell
./scripts/compiler1.sh
```

**PowerShell**
```Shell
.\scripts\compiler1.ps1
```

Produces:

serialized_ast.json

---

#### Level 2 Commands
##### Python → Rust Handoff Validation
**Bash**
```Shell
./scripts/compiler2.sh
```

**PowerShell**
```Shell
.\scripts\compiler2.ps1
```

Produces:

serialized_ast.json

backend_output.*

---

#### Level 3 Commands
##### **Nim**-*Positive Tests:*

*Bash*
```bash
./scripts/nim3+.sh
```
*PowerShell*
```pwsh
.\scripts\nim3+.ps1
```
##### **Nim**-*Negative Tests:*

*Bash*
```bash
./scripts/nim3-.sh
```
*PowerShell*
```pwsh
.\scripts\nim3-.ps1
```
##### **Nim**-*Specialty Tests:*

*Bash*
```shell
./scripts/nim3s.sh
```

*PowerShell*
```shell
.\scripts\nim3s.ps1
```

##### **D**
*Positive*
```Shell
python run_LVL3_full_positive_d_test.py
```

*Negative*
```Shell

python run_LVL3_full_negative_d_test.py
```


##### **Elixir**
*Positive*
```Shell
python run_LVL3_full_positive_ex_test.py
```

*Negative*
```Shell
python run_LVL3_full_negative_ex_test.py
```
---

#### Level 4 Commands
##### **Nim**-*Positive Batch Tests:*

*Bash*
```Shell
./scripts/nim4+.sh
```
*PowerShell*
```shell
.\scripts\nim4+.ps1
```

##### **Nim**-*Negative Batch Tests:*

*Bash*
```Shell
./scripts/nim4-.sh
```

*PowerShell*
```shell
.\scripts\nim4-.ps1
```

##### **Nim**-*Specialty Batch Tests:*

*Bash*
```shell
./scripts/nim4s.sh
```

*PowerShell*
```shell
.\scripts\nim4s.ps1
```

##### **D**
*Positive Batch*
```Shell
python run_LVL4_pos_d_batch_tests.py
```

*Negative Batch*
```Shell
python run_LVL4_neg_d_batch_tests.py
```

##### **Elixir**
*Positive Batch*
```Shell
python run_LVL4_pos_ex_batch_tests.py
```

*Negative Batch*
```Shell
python run_LVL4_neg_ex_batch_tests.py
```

### Recommended Execution Order
##### **Multiple Positive Tests**

Level 4 Positive
    ↓
Review Failures
    ↓
Level 3 Positive
    ↓
Debug Failures


##### **Multiple Negative Tests**

Level 4 Negative
    ↓
Review Unexpected Passes
    ↓
Level 3 Negative
    ↓
Debug Failures


## Underlying test runners

The wrapper scripts execute the following Python runners in both 'Python' and 'Python3':


**Level 1**

--------
```Shell
python run_LVL1_compiler_test.py
```
 

**Level 2**

--------
```Shell
python run_LVL2_handoff_test.py
```
 

**Level 3**

--------
```Shell
python run_LVL3_full_positive_nim_test.py

python run_LVL3_full_negative_nim_test.py

python run_LVL3_nim_specialty.py

python run_LVL3_full_positive_d_test.py

python run_LVL3_full_negative_d_test.py

python run_LVL3_full_positive_ex_test.py

python run_LVL3_full_negative_ex_test.py
```
 

**Level 4**

--------
```Shell
python run_LVL4_pos_nim_batch_tests.py

python run_LVL4_neg_nim_batch_tests.py

python run_LVL4_nim_btch_spcl.py

python run_LVL4_pos_d_batch_tests.py

python run_LVL4_neg_d_batch_tests.py

python run_LVL4_pos_ex_batch_tests.py

python run_LVL4_neg_ex_batch_tests.py
```