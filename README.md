# Oatool
[![Build Status](https://secure.travis-ci.org/JordiPolo/oatool.svg)](https://travis-ci.org/JordiPolo/oatool)
[![Windows Build status](https://ci.appveyor.com/api/projects/status/6uet336897fjowet/branch/master?svg=true)](https://ci.appveyor.com/project/JordiPolo/oatool/branch/master)


Oatool is a command line tool to work with [OpenApi specification](https://github.com/OAI/OpenAPI-Specification/) files.

It can be used to convert OpenAPI files to Google discovery files and viceversa.

## Installing

To install the Rust toolchain and run the code, do:

```
curl https://sh.rustup.rs -sSf | sh
rustup update
cargo install
cargo build
cargo run -- <one of the commands below>
```

If you do not want to setup Rust, this repository contains static linked binaries which should run without dependencies in Linux, MacOS X and Windows. Just download the [latest release](https://github.com/JordiPolo/oatool/releases) for your platform.

## Using

### Validate OpenAPI file
```
oatool validate openapi.yaml
```

### Validate OpenAPI file which is convertible to the Google Discovery format
```
oatool validate openapi.yaml --support_google
```


### Convert to JSON
```
oatool convert openapi.yaml --from=openapi --to=openapi_json
```

### Convert to yaml
```
oatool convert openapi.json --from=openapi --to=openapi_yaml
```

### Convert from Google Discovery to OpenAPI
```
oatool convert google_discovery_spec.yml --from=google --to=openapi_yaml
```

### Convert from OpenAPI to Google Discovery
```
oatool convert openapi.yaml --from=openapi --to=google
```


All operations print to stdout. Output can be redirected to an output file:
```
oatool convert openapi.yaml --from=openapi --to=google > google_discovery_spec.yaml
```




## TODO

* Make validation rules explicit, maybe add profiles or CLI flags to control them.
* Convert from/to other formats: Openapi 3, etc.
* Add many test documents
* Support $ref inter documents
* Support <<*
* Smaller executable
* SLT format in openapi
