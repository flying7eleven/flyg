#!/bin/bash
curl --silent -X POST -H 'Content-Type: application/json' --data '{"username":"demo@example.com","password":"demopassword"}' http://localhost:8000/v1/auth/token | json_pp
