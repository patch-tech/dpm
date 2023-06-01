// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var dpm_agent_pb = require('./dpm_agent_pb.js');

function serialize_CompiledQuery(arg) {
  if (!(arg instanceof dpm_agent_pb.CompiledQuery)) {
    throw new Error('Expected argument of type CompiledQuery');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_CompiledQuery(buffer_arg) {
  return dpm_agent_pb.CompiledQuery.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_ConnectionRequest(arg) {
  if (!(arg instanceof dpm_agent_pb.ConnectionRequest)) {
    throw new Error('Expected argument of type ConnectionRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_ConnectionRequest(buffer_arg) {
  return dpm_agent_pb.ConnectionRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_ConnectionResponse(arg) {
  if (!(arg instanceof dpm_agent_pb.ConnectionResponse)) {
    throw new Error('Expected argument of type ConnectionResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_ConnectionResponse(buffer_arg) {
  return dpm_agent_pb.ConnectionResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_Query(arg) {
  if (!(arg instanceof dpm_agent_pb.Query)) {
    throw new Error('Expected argument of type Query');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_Query(buffer_arg) {
  return dpm_agent_pb.Query.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_QueryResult(arg) {
  if (!(arg instanceof dpm_agent_pb.QueryResult)) {
    throw new Error('Expected argument of type QueryResult');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_QueryResult(buffer_arg) {
  return dpm_agent_pb.QueryResult.deserializeBinary(new Uint8Array(buffer_arg));
}


// The `dpm-agent` service enables connecting to several cloud DB backends,
// compiling, and executing queries on these backends.
var DpmAgentService = exports.DpmAgentService = {
  // Create connection to a supported cloud DB backend.
createConnection: {
    path: '/DpmAgent/CreateConnection',
    requestStream: false,
    responseStream: false,
    requestType: dpm_agent_pb.ConnectionRequest,
    responseType: dpm_agent_pb.ConnectionResponse,
    requestSerialize: serialize_ConnectionRequest,
    requestDeserialize: deserialize_ConnectionRequest,
    responseSerialize: serialize_ConnectionResponse,
    responseDeserialize: deserialize_ConnectionResponse,
  },
  // Compile a query on the selected backend.
compileQuery: {
    path: '/DpmAgent/CompileQuery',
    requestStream: false,
    responseStream: false,
    requestType: dpm_agent_pb.Query,
    responseType: dpm_agent_pb.CompiledQuery,
    requestSerialize: serialize_Query,
    requestDeserialize: deserialize_Query,
    responseSerialize: serialize_CompiledQuery,
    responseDeserialize: deserialize_CompiledQuery,
  },
  // Execute a query on the selected backend.
executeQuery: {
    path: '/DpmAgent/ExecuteQuery',
    requestStream: false,
    responseStream: false,
    requestType: dpm_agent_pb.Query,
    responseType: dpm_agent_pb.QueryResult,
    requestSerialize: serialize_Query,
    requestDeserialize: deserialize_Query,
    responseSerialize: serialize_QueryResult,
    responseDeserialize: deserialize_QueryResult,
  },
};

exports.DpmAgentClient = grpc.makeGenericClientConstructor(DpmAgentService);
