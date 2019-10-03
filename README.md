# Ripsaw

![logo](img/logo.jpg?raw=true "Ripsaw")


A tool split CSV file by rows count with its header.

This works almost same as a following AWK script.
\# `l` (line numbers count) must be more than 2

```zsh
% awk -v p="result-" -v s=".csv" -v l="10000" -f ripsaw.awk input.csv
```

```zsh
% ripsaw.sh input.csv result 10000
```

## Setup

### Standalone CLI

TODO

### Server

```zsh
% python2.7 -m virtualenv venv
% source venv/bin/activate
(venv) % ./tool/setup-cloud-sdk
(venv) % source ./tool/load-gcloud
(venv) % gcloud init
```

TODO

## License

This project is distributed under the license.

```
Ripsaw
Copyright 2019 Yasuhiro Яша Asaka
```

`Apache-2.0`

```
Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```

See [LICENSE](LICENSE).
