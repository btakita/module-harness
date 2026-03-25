"""
# Module: utils

## Spec
- Parse JSON input strings
- Validate schema against template

## Evals
- parse_valid: valid JSON string → parsed dict
"""

import json

def parse_input(data: str) -> dict:
    return json.loads(data)
