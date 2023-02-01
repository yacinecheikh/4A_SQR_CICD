# 4A_SQR_CICD
CI/CD repository for SQR students [Ahmed Sguiar](https://github.com/SguiarAhmed) and [Yacine Cheikhrouhou](https://github.com/yacinecheikh)

[![forthebadge made-with-rust](http://ForTheBadge.com/images/badges/made-with-rust.svg)](https://www.rust-lang.org)

### CI Badges

![badge echo on push](https://github.com/yacinecheikh/4A_SQR_CICD/actions/workflows/echo.yml/badge.svg)
![badge script on pull](https://github.com/yacinecheikh/4A_SQR_CICD/actions/workflows/onpull.yml/badge.svg)
![badge curl moon](https://github.com/yacinecheikh/4A_SQR_CICD/actions/workflows/mooncurl.yml/badge.svg)

The Swagger documentation is written in the resources/openapi.yaml file, and can be opened at [The Swagger Website](https://editor.swagger.io/)

## For the teacher

### Project choices
Instead of using Python with Flask, we decided to use Rust with Rocket in order to learn a new programming language.
We decided to choose the guided project.

### Loading an example database

The CSV upload endpoint is not available.
Instead, the following curl commands can be used to register transactions:
```console
# receive money from the bank
curl localhost:8000 -X POST -d "sender=mybank&receiver=me&amount=1000"
# receive double the amount by mistake
curl localhost:8000 -X POST -d "sender=mybank&receiver=me&amount=1000"
# wait between transactions to have different timestamps
sleep 2
# send back the received amount
curl localhost:8000 -X POST -d "sender=me&receiver=mybank&amount=1000"
```

