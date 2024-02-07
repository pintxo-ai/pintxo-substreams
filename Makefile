ENDPOINT ?= mainnet.eth.streamingfast.io:443
START_BLOCK ?= 12292922
STOP_BLOCK ?= +10

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run
run: build
	substreams run -e eth.substreams.pinax.network:443 substreams.yaml map_seaport_purchases -s -10

.PHONY: gui
gui: build
	substreams gui -e $(ENDPOINT) substreams.yaml db_out -s $(START_BLOCK) -t $(STOP_BLOCK)

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: pack
pack: build
	substreams pack substreams.yaml

.PHONY: test
test: build
	substreams run -e eth.substreams.pinax.network:443 substreams.yaml map_seaport_purchases -s 19178136 -t 19178140 

.PHONY: uniswap
uniswap: build
	substreams run -e eth.substreams.pinax.network:443 substreams.yaml map_pools_created -s -10 

.PHONY: metrics
metrics: build
	substreams run -e eth.substreams.pinax.network:443 substreams.yaml metrics_out -s -10