# Flyg
TODO

## Build status
| Package        | Build status (main branch)                                                                                    | Notes                     |
| :------------- | :-----------------------------------------------------------------------------------------------------------: | :------------------------ |
|  flyg-core     | ![flyg-core](https://github.com/flying7eleven/flyg/workflows/flyg-core%20(build)/badge.svg?branch=main)       | -                         |
|  flyg-ms2020   | :grey_question:                                                                                               | Requires the MSFS2020 SDK |
|  flyg-backend  | ![flyg-backend](https://github.com/flying7eleven/flyg/workflows/flyg-backend%20(build)/badge.svg?branch=main) | -                         |
|  flyg-client   | :grey_question:                                                                                               | Depends on flyg-ms2020    |

## Security audit
The result of the latest automated security audit: ![flyg project (security audit)](https://github.com/flying7eleven/flyg/workflows/flyg%20project%20(security%20audit)/badge.svg?branch=main)

## Generate the signing keys for the JWT token
The JWT token is signed with an asymmetric RSA key. To generate this key, execute the following command in the root folder of the repository:

1. Generate a RSA key pair by executing `openssl genrsa -out jwt_token_private.pem 4096`
2. Extract the public key via `openssl rsa -in jwt_token_private.pem -out jwt_token_public.pem -pubout -outform PEM`

## Test data
(51.275397228764334, 6.752361168007028) -> EDDL should be the closest (is a parking position)
