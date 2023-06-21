// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var dpm_agent_pb = require('./dpm_agent_pb.js');

function serialize_dpm_agent_CompiledQuery(arg) {
  if (!(arg instanceof dpm_agent_pb.CompiledQuery)) {
    throw new Error('Expected argument of type dpm_agent.CompiledQuery');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_dpm_agent_CompiledQuery(buffer_arg) {
  return dpm_agent_pb.CompiledQuery.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_dpm_agent_ConnectionRequest(arg) {
  if (!(arg instanceof dpm_agent_pb.ConnectionRequest)) {
    throw new Error('Expected argument of type dpm_agent.ConnectionRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_dpm_agent_ConnectionRequest(buffer_arg) {
  return dpm_agent_pb.ConnectionRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_dpm_agent_ConnectionResponse(arg) {
  if (!(arg instanceof dpm_agent_pb.ConnectionResponse)) {
    throw new Error('Expected argument of type dpm_agent.ConnectionResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_dpm_agent_ConnectionResponse(buffer_arg) {
  return dpm_agent_pb.ConnectionResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_dpm_agent_DisconnectRequest(arg) {
  if (!(arg instanceof dpm_agent_pb.DisconnectRequest)) {
    throw new Error('Expected argument of type dpm_agent.DisconnectRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_dpm_agent_DisconnectRequest(buffer_arg) {
  return dpm_agent_pb.DisconnectRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_dpm_agent_DisconnectResponse(arg) {
  if (!(arg instanceof dpm_agent_pb.DisconnectResponse)) {
    throw new Error('Expected argument of type dpm_agent.DisconnectResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_dpm_agent_DisconnectResponse(buffer_arg) {
  return dpm_agent_pb.DisconnectResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

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
  // Create connection to a supported cloud DB backend.
createConnection: {
    path: '/dpm_agent.DpmAgent/CreateConnection',
    requestStream: false,
    responseStream: false,
    requestType: dpm_agent_pb.ConnectionRequest,
    responseType: dpm_agent_pb.ConnectionResponse,
    requestSerialize: serialize_dpm_agent_ConnectionRequest,
    requestDeserialize: deserialize_dpm_agent_ConnectionRequest,
    responseSerialize: serialize_dpm_agent_ConnectionResponse,
    responseDeserialize: deserialize_dpm_agent_ConnectionResponse,
  },
  // Compile a query on the selected backend.
compileQuery: {
    path: '/dpm_agent.DpmAgent/CompileQuery',
    requestStream: false,
    responseStream: false,
    requestType: dpm_agent_pb.Query,
    responseType: dpm_agent_pb.CompiledQuery,
    requestSerialize: serialize_dpm_agent_Query,
    requestDeserialize: deserialize_dpm_agent_Query,
    responseSerialize: serialize_dpm_agent_CompiledQuery,
    responseDeserialize: deserialize_dpm_agent_CompiledQuery,
  },
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
  // Disconnect connection.
disconnectConnection: {
    path: '/dpm_agent.DpmAgent/DisconnectConnection',
    requestStream: false,
    responseStream: false,
    requestType: dpm_agent_pb.DisconnectRequest,
    responseType: dpm_agent_pb.DisconnectResponse,
    requestSerialize: serialize_dpm_agent_DisconnectRequest,
    requestDeserialize: deserialize_dpm_agent_DisconnectRequest,
    responseSerialize: serialize_dpm_agent_DisconnectResponse,
    responseDeserialize: deserialize_dpm_agent_DisconnectResponse,
  },
};

exports.DpmAgentClient = grpc.makeGenericClientConstructor(DpmAgentService);
