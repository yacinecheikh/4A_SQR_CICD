# 4A_SQR_CICD
CI/CD repository for SQR students [Ahmed Sguiar](https://github.com/SguiarAhmed) and [Yacine Cheikhrouhou](https://github.com/yacinecheikh)

[![forthebadge made-with-rust](http://ForTheBadge.com/images/badges/made-with-rust.svg)](https://www.rust-lang.org)
Le projet de CI/CD est un [![](https://img.shields.io/badge/PROJET_TERMINÉ_🚀-059142?style=for-the-badge&logoColor=white)](https://dev.to/envoy_/150-badges-for-github-pnk) 

### CI Badges

![badge echo on push](https://github.com/yacinecheikh/4A_SQR_CICD/actions/workflows/echo.yml/badge.svg)
![badge script on pull](https://github.com/yacinecheikh/4A_SQR_CICD/actions/workflows/onpull.yml/badge.svg)
![badge curl moon](https://github.com/yacinecheikh/4A_SQR_CICD/actions/workflows/mooncurl.yml/badge.svg)

The Swagger documentation is written in the resources/openapi.yaml file, and can be opened at [The Swagger Website](https://editor.swagger.io/)

## API

### curl examples

#### route /transactions/transaction
```bash
curl -X POST localhost:8000/transactions/transaction
    -d "sender=mybank&receiver=me&amount=1000"
```


#### route /transactions/list
```bash
curl -X GET localhost:8000/transactions/list
```
```
1675267138: mybank -> me (1000)
```


#### route /transactions/list/<user>
```bash
curl -X GET localhost:8000/transactions/list/me
```
```text
1675267138: mybank -> me (1000)
```

#### route /transactions/balance/user
```bash
curl -X GET localhost:8000/transactions/balance/me
```
```text
1000
```

#### route /transactions/integrity_check
note: this route does not work currently
the hashing verification always fail, even though the inputs should be the same
```bash
curl -X GET localhost:8000/transactions/integrity_check
```



## For the teacher

### Project choices
Instead of using Python with Flask, we decided to use Rust with Rocket in order to learn a new programming language.
For the Dockerfile, we used [this reference](https://www.koyeb.com/tutorials/deploy-a-rust-web-app-with-rocket)
We decided to choose the guided project.

The hashing function used is the standard rust hash algorithm.
The std::hash module provides a macro to automatically hash any data structure from its memory layout.
The implementation details can be found [here](https://nnethercote.github.io/2021/12/08/a-brutally-effective-hash-function-in-rust.html)



### Loading an example database

The CSV upload endpoint is currently not available.
Instead, the following curl commands can be used to register transactions:
```bash
# receive money from the bank
curl localhost:8000 -X POST -d "sender=mybank&receiver=me&amount=1000"
# receive double the amount by mistake
curl localhost:8000 -X POST -d "sender=mybank&receiver=me&amount=1000"
# wait between transactions to have different timestamps
sleep 2
# send back the received amount
curl localhost:8000 -X POST -d "sender=me&receiver=mybank&amount=1000"
```

