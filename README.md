# Oatool
[![Build Status](https://secure.travis-ci.org/JordiPolo/oatool.svg)](https://travis-ci.org/JordiPolo/oatool)
[![Windows Build status](https://ci.appveyor.com/api/projects/status/6uet336897fjowet/branch/master?svg=true)](https://ci.appveyor.com/project/JordiPolo/oatool/branch/master)


Oatool is a command line tool to work with [OpenApi specification](https://github.com/OAI/OpenAPI-Specification/) files.

Currently it can validate their correctness and convert to/from JSON and YAML.
More functionality like merging diferent files will be added soon.

## Installing

TODO: Install via Cargo.

If you do not want to setup Rust, this repository contains static linked binaries which should run without dependencies in Linux, MacOS X and Windows. Just download the [latest release](./releases) for your platform.

## Using

### Validate
```
oatool validate < openapi.yaml
```

### Convert to JSON
```
oatool to_json < openapi.yaml
```

### Convert to yaml
```
oatool to_json < openapi.yaml
```

All operations act upon stdin and print to stdout so they can be redirected to files or use them together:
```
oatool to_json < openapi.yaml > openapi.json
```


## TODO

* Make validation rules explicit, maybe add profiles or CLI flags to control them.
* Convert from/to other formats : Openapi 3, google discovery, etc.
* Many test documents
* Optionally act upon list of files