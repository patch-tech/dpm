// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var dpm_agent_pb = require('./dpm_agent_pb.js');

function serialize_dpm_agent_Query(arg) {
  if (!(arg instanceof dpm_agent_pb.Query)) {
    throw new Error('Expected argument of type dpm_agent.Query');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_dpm_agent_Query(buffer_arg) {
  return dpm_agent_pb.Query.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_dpm_agent_QueryResult(arg) {
  if (!(arg instanceof dpm_agent_pb.QueryResult)) {
    throw new Error('Expected argument of type dpm_agent.QueryResult');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_dpm_agent_QueryResult(buffer_arg) {
  return dpm_agent_pb.QueryResult.deserializeBinary(new Uint8Array(buffer_arg));
}


// The `dpm-agent` service enables connecting to several cloud DB backends,
// compiling, and executing queries on these backends.
var DpmAgentService = exports.DpmAgentService = {
  // Execute a query on the selected backend.
executeQuery: {
    path: '/dpm_agent.DpmAgent/ExecuteQuery',
    requestStream: false,
    responseStream: false,
    requestType: dpm_agent_pb.Query,
    responseType: dpm_agent_pb.QueryResult,
    requestSerialize: serialize_dpm_agent_Query,
    requestDeserialize: deserialize_dpm_agent_Query,
    responseSerialize: serialize_dpm_agent_QueryResult,
    responseDeserialize: deserialize_dpm_agent_QueryResult,
  },
};

exports.DpmAgentClient = grpc.makeGenericClientConstructor(DpmAgentService);
