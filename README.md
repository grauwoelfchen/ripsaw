# Ripsaw

![logo](img/logo.jpg?raw=true "Ripsaw")


A tool/server splitting CSV file by rows count with its header.  
You would not need sharp saw any more!


## Idea

This works almost just same as a following AWK script.

```zsh
% cat ./ripsaw.awk
NR == 1 { header = $0; next }
NR % l == 2 { close(x); N++; x = p N s; print header > x }
{ print > x }
```

\# `l` (line numbers count) must be more than 2

```zsh
% awk -v p="result-" -v s=".csv" -v l="10000" -f ripsaw.awk input.csv
```

```zsh
% ripsaw.sh input.csv result 10000
```

##### Requirements

* Rust
* MUSL libc (only for server)


## Setup

### Standalone CLI

TODO

### Server

##### Local

```zsh
% make
% HOST=127.0.0.1 PORT=8000 ./ripsaw
```

##### Docker

```zsh
% docker build . -t ripsaw:latest
% docker run --env PORT=8000 -p 127.0.0.1:8000:8000/tcp ripsaw
```

##### e.g. Cloud Pub/Sub + Cloud Run + Cloud Storage

```zsh
% python2.7 -m virtualenv venv
% source venv/bin/activate
(venv) % ./tool/setup-cloud-sdk
(venv) % source ./tool/load-gcloud
(venv) % gcloud init
```

##### Cloud Pub/Sub

Just create a service account and a pub/sub topic we want to subscribe.

```zsh
(venv) % gcloud projects add-iam-policy-binding <PROJECT-ID> \
  --member=serviceAccount:service-<PROJECT-NUMBER>@gcp-sa-pubsub.iam.gserviceaccount.com \
  --role=roles/iam.serviceAccountTokenCreator
(venv) % gcloud iam service-accounts create cloud-run-ripsaw-invoker \
  --display-name "Cloud Run Ripsaw Invoker"

(venv) % gcloud beta run services add-iam-policy-binding ripsaw \
 --member=serviceAccount:cloud-run-ripsaw-invoker@<PROJECT-ID>.iam.gserviceaccount.com \
 --role=roles/run.invoker

(venv) % gcloud beta pubsub topics create ripsaw
(venv) % gcloud beta pubsub subscriptions create ripsaw-subscriptions \
  --topic ripsaw \
  --push-endpoint=https://<DOMAIN>/ \
  --push-auth-service-account=cloud-run-ripsaw-invoker@<PROJECT-ID>.iam.gserviceaccount.com
```

##### Cloud Storage

Let's create a bucket and configure notification to topic we've just created.

```zsh
(venv) % gsutil mb --retention 1d gs://<BUCKET-NAME>
(venv) % gsutil notification create -f json \
  -t projects/<PROJECT-ID>/topics/ripsaw gs://<BUCKET-NAME>
```

###### Cloud Run

Finally, build an image using Dockerfile and then deploy it on the cluster.

```zsh
(venv) % gcloud config set run/region <REGION>

(venv) % gcloud builds submit --tag gcr.io/<PROJECT-ID>/ripsaw
(venv) % gcloud beta run deploy ripsaw \
  --image gcr.io/<PROJECT-ID>/ripsaw --platform managed
```


## Todos

* [ ] Support compressed files (by zip or gz etc. for input/output)
* [ ] Set lifecycle rule
* [ ] Enable splitting by data size (not rows count)
* [ ] Consider error report
* [ ] Think something about limitation (data size?)
* [ ] Monitor splitting progress
* [ ] Support .xlsx?

And... only for server:

* [ ] Support short-term file retention (currently 1 day by the bucket)
* [ ] Create Web UI (upload/download)?


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
