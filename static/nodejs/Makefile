TARGET=src/backends/dpm_agent

gen: gen-js gen-ts
	echo "Generated js and ts"

gen-js:
	./node_modules/.bin/grpc_tools_node_protoc \
	--js_out=import_style=commonjs,binary:${TARGET} \
	--grpc_out=grpc_js:${TARGET} \
	-I ../../proto/ \
	../..//proto/dpm_agent.proto

gen-ts:
	./node_modules/.bin/grpc_tools_node_protoc \
	--ts_out=grpc_js:${TARGET} \
	--plugin=protoc-gen-ts=./node_modules/.bin/protoc-gen-ts \
	-I ../../proto/ \
	../..//proto/dpm_agent.proto
