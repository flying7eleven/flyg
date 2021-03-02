#!/bin/bash
curl --silent -X GET -H 'Content-Type: application/json' -H "Authorization: Bearer $1" http://localhost:8000/v1/airports/closest?latitude\=51.230278\&longitude\=6.504444 | json_pp
