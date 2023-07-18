TARGET=src/backends/dpm_agent
gen:
	 poetry run python \
	 -m grpc_tools.protoc \
	 -I../../proto \
	 --python_out=${TARGET} \
	 --pyi_out=${TARGET} \
	 --grpc_python_out=${TARGET} \
	 ../../proto/dpm_agent.proto
