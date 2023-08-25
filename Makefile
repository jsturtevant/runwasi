PREFIX ?= /usr/local
INSTALL ?= install
TEST_IMG_NAME ?= wasmtest:latest
RUNTIMES ?= wasmedge wasmtime
export CONTAINERD_NAMESPACE ?= default
SUDO ?= sudo

TARGET ?= debug
RELEASE_FLAG :=
ifeq ($(TARGET),release)
RELEASE_FLAG = --release
endif

RUN_OS := unix
FEATURES := --features libcontainer_default
WARNINGS := -D warnings
ifeq ($(OS), Windows_NT)
# need to turn off static/standalone for wasm-edge
RUN_OS = windows
PREFIX = "C:\Users\jstur\projects\containerd\bin"
SUDO= 
FEATURES = --no-default-features
# turn of warnings until windows is fully supported #49
WARNINGS = 
endif

DOCKER_BUILD ?= docker buildx build

KIND_CLUSTER_NAME ?= containerd-wasm

.PHONY: build
build:
	cargo build -p containerd-shim-wasm --features generate_bindings $(RELEASE_FLAG)
	cargo build -p containerd-shim-wasm $(FEATURES) $(RELEASE_FLAG)
	cargo build $(FEATURES) $(RELEASE_FLAG)

.PHONY: check
check:
	cargo +nightly fmt --all -- --check
	cargo clippy $(FEATURES) --all --all-targets -- $(WARNINGS)

.PHONY: fix
fix:
	cargo +nightly fmt --all
	cargo clippy $(FEATURES) --fix --all --all-targets -- $(WARNINGS)

.PHONY: test
test:
	RUST_LOG=trace cargo test $(FEATURES) --all --verbose -- --nocapture
ifneq ($(OS), Windows_NT)
	# run wasmedge test without the default `static` feature
	RUST_LOG=trace cargo test --package containerd-shim-wasmedge --verbose --no-default-features --features standalone -- --nocapture
endif

.PHONY: install
install: build
	$(MAKE) install-$(RUN_OS)

.PHONY: install-unix
install-unix:
	mkdir -p $(PREFIX)/bin
	$(foreach runtime,$(RUNTIMES), \
		$(INSTALL) target/$(TARGET)/containerd-shim-$(runtime)-v1 $(PREFIX)/bin/; \
		$(INSTALL) target/$(TARGET)/containerd-shim-$(runtime)d-v1 $(PREFIX)/bin/; \
		$(INSTALL) target/$(TARGET)/containerd-$(runtime)d $(PREFIX)/bin/; \
	)

.PHONY: install-windows
install-windows:
	$(foreach runtime,$(RUNTIMES), \
		cp target/$(TARGET)/containerd-shim-$(runtime)-v1.exe $(PREFIX); \
		cp target/$(TARGET)/containerd-$(runtime)-windows.exe $(PREFIX); \
	)

dist:
	$(MAKE) install PREFIX=$(PWD)/dist RUNTIMES=$(RUNTIMES) TARGET=$(TARGET)

.PHONY: test-image
test-image: dist/img.tar

.PHONY: test-image
test-image/clean:
	rm -rf target/wasm32-wasi/$(TARGET)/

.PHONY: target/wasm32-wasi/$(TARGET)/wasi-demo-app.wasm
target/wasm32-wasi/$(TARGET)/wasi-demo-app.wasm:
	rustup target add wasm32-wasi
	cd crates/wasi-demo-app && cargo build $(RELEASE_FLAG)

target/wasm32-wasi/$(TARGET)/img.tar: target/wasm32-wasi/$(TARGET)/wasi-demo-app.wasm
	cd crates/wasi-demo-app && cargo build $(RELEASE_FLAG) --features oci-v1-tar

dist/img.tar: target/wasm32-wasi/$(TARGET)/img.tar
	@mkdir -p "dist/"
	cp "$<" "$@"

load: dist/img.tar
	$(SUDO) ctr -n $(CONTAINERD_NAMESPACE) image import --all-platforms $<

bin/kind: test/k8s/Dockerfile
	$(DOCKER_BUILD) --output=bin/ -f test/k8s/Dockerfile --target=kind .

test/k8s/_out/img: test/k8s/Dockerfile dist
	mkdir -p $(@D) && $(DOCKER_BUILD) -f test/k8s/Dockerfile --iidfile=$(@) --load  .

.PHONY: test/nginx
test/nginx:
	docker pull docker.io/nginx:latest
	mkdir -p $@/out && docker save -o $@/out/img.tar docker.io/nginx:latest

.PHONY: test/k8s/cluster
test/k8s/cluster: dist/img.tar bin/kind test/k8s/_out/img
	bin/kind create cluster --name $(KIND_CLUSTER_NAME) --image="$(shell cat test/k8s/_out/img)" && \
	bin/kind load image-archive --name $(KIND_CLUSTER_NAME) $(<)

.PHONY: test/k8s
test/k8s: test/k8s/cluster
	kubectl --context=kind-$(KIND_CLUSTER_NAME) apply -f test/k8s/deploy.yaml
	kubectl --context=kind-$(KIND_CLUSTER_NAME) wait deployment wasi-demo --for condition=Available=True --timeout=90s

.PHONY: test/k8s/clean
test/k8s/clean: bin/kind
	bin/kind delete cluster --name $(KIND_CLUSTER_NAME)

.PHONY: bin/k3s
bin/k3s:
	mkdir -p bin && \
	curl -sfL https://get.k3s.io | INSTALL_K3S_BIN_DIR=$(PWD)/bin INSTALL_K3S_SYMLINK=skip INSTALL_K3S_NAME=runwasi sh -

.PHONY: bin/k3s/clean
bin/k3s/clean:
	bin/k3s-runwasi-uninstall.sh

.PHONY: test/k3s
test/k3s: dist/img.tar bin/k3s dist
	sudo cp /var/lib/rancher/k3s/agent/etc/containerd/config.toml /var/lib/rancher/k3s/agent/etc/containerd/config.toml.tmpl && \
	echo '[plugins."io.containerd.grpc.v1.cri".containerd.runtimes.wasm]' | sudo tee -a /var/lib/rancher/k3s/agent/etc/containerd/config.toml.tmpl && \
	echo '  runtime_type = "$(PWD)/dist/bin/containerd-shim-wasmedge-v1"' | sudo tee -a /var/lib/rancher/k3s/agent/etc/containerd/config.toml.tmpl && \
	echo "CONTAINERD_NAMESPACE='default'" | sudo tee /etc/systemd/system/k3s-runwasi.service.env && \
	echo "NO_PROXY=192.168.0.0/16" | sudo tee -a /etc/systemd/system/k3s-runwasi.service.env && \
	sudo systemctl daemon-reload && \
	sudo systemctl restart k3s-runwasi && \
	timeout 60 bash -c -- 'while true; do sudo bin/k3s ctr version && break; sleep 1; done' && \
	sudo bin/k3s ctr image import --all-platforms dist/img.tar && \
	sudo bin/k3s kubectl apply -f test/k8s/deploy.yaml
	sudo bin/k3s kubectl wait deployment wasi-demo --for condition=Available=True --timeout=90s && \
	sudo bin/k3s kubectl get pods -o wide

.PHONY: test/k3s/clean
test/k3s/clean: bin/k3s/clean;
