NAME ?= ripsaw
IMAGE ?= gcr.io/$(PROJECT_ID)/$(NAME)

build:
	cargo build
.PHONY: build

build\:release:
	cargo build --release --bin $(NAME) \
	  --target x86_64-unknown-linux-musl
.PHONY: build\:release

deploy:
	gcloud builds submit --tag $(IMAGE) --timeout 1h
	gcloud beta run deploy $(NAME) \
		--image $(IMAGE) \
		--platform managed
.PHONY: build

.DEFAULT_GOAL = build
