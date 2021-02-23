#!/bin/bash
curl --verbose -X GET -H 'Content-Type: application/json' -H "Authorization: Bearer $1" http://localhost:8000/v1/airports/EDDL | json_pp
