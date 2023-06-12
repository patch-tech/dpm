// package: dpm_agent
// file: dpm_agent.proto

/* tslint:disable */
/* eslint-disable */

import * as grpc from "@grpc/grpc-js";
import * as dpm_agent_pb from "./dpm_agent_pb";

interface IDpmAgentService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
    createConnection: IDpmAgentService_ICreateConnection;
    compileQuery: IDpmAgentService_ICompileQuery;
    executeQuery: IDpmAgentService_IExecuteQuery;
}

interface IDpmAgentService_ICreateConnection extends grpc.MethodDefinition<dpm_agent_pb.ConnectionRequest, dpm_agent_pb.ConnectionResponse> {
    path: "/dpm_agent.DpmAgent/CreateConnection";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<dpm_agent_pb.ConnectionRequest>;
    requestDeserialize: grpc.deserialize<dpm_agent_pb.ConnectionRequest>;
    responseSerialize: grpc.serialize<dpm_agent_pb.ConnectionResponse>;
    responseDeserialize: grpc.deserialize<dpm_agent_pb.ConnectionResponse>;
}
interface IDpmAgentService_ICompileQuery extends grpc.MethodDefinition<dpm_agent_pb.Query, dpm_agent_pb.CompiledQuery> {
    path: "/dpm_agent.DpmAgent/CompileQuery";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<dpm_agent_pb.Query>;
    requestDeserialize: grpc.deserialize<dpm_agent_pb.Query>;
    responseSerialize: grpc.serialize<dpm_agent_pb.CompiledQuery>;
    responseDeserialize: grpc.deserialize<dpm_agent_pb.CompiledQuery>;
}
interface IDpmAgentService_IExecuteQuery extends grpc.MethodDefinition<dpm_agent_pb.Query, dpm_agent_pb.QueryResult> {
    path: "/dpm_agent.DpmAgent/ExecuteQuery";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<dpm_agent_pb.Query>;
    requestDeserialize: grpc.deserialize<dpm_agent_pb.Query>;
    responseSerialize: grpc.serialize<dpm_agent_pb.QueryResult>;
    responseDeserialize: grpc.deserialize<dpm_agent_pb.QueryResult>;
}

export const DpmAgentService: IDpmAgentService;

export interface IDpmAgentServer extends grpc.UntypedServiceImplementation {
    createConnection: grpc.handleUnaryCall<dpm_agent_pb.ConnectionRequest, dpm_agent_pb.ConnectionResponse>;
    compileQuery: grpc.handleUnaryCall<dpm_agent_pb.Query, dpm_agent_pb.CompiledQuery>;
    executeQuery: grpc.handleUnaryCall<dpm_agent_pb.Query, dpm_agent_pb.QueryResult>;
}

export interface IDpmAgentClient {
    createConnection(request: dpm_agent_pb.ConnectionRequest, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.ConnectionResponse) => void): grpc.ClientUnaryCall;
    createConnection(request: dpm_agent_pb.ConnectionRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.ConnectionResponse) => void): grpc.ClientUnaryCall;
    createConnection(request: dpm_agent_pb.ConnectionRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.ConnectionResponse) => void): grpc.ClientUnaryCall;
    compileQuery(request: dpm_agent_pb.Query, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.CompiledQuery) => void): grpc.ClientUnaryCall;
    compileQuery(request: dpm_agent_pb.Query, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.CompiledQuery) => void): grpc.ClientUnaryCall;
    compileQuery(request: dpm_agent_pb.Query, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.CompiledQuery) => void): grpc.ClientUnaryCall;
    executeQuery(request: dpm_agent_pb.Query, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.QueryResult) => void): grpc.ClientUnaryCall;
    executeQuery(request: dpm_agent_pb.Query, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.QueryResult) => void): grpc.ClientUnaryCall;
    executeQuery(request: dpm_agent_pb.Query, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.QueryResult) => void): grpc.ClientUnaryCall;
}

export class DpmAgentClient extends grpc.Client implements IDpmAgentClient {
    constructor(address: string, credentials: grpc.ChannelCredentials, options?: Partial<grpc.ClientOptions>);
    public createConnection(request: dpm_agent_pb.ConnectionRequest, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.ConnectionResponse) => void): grpc.ClientUnaryCall;
    public createConnection(request: dpm_agent_pb.ConnectionRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.ConnectionResponse) => void): grpc.ClientUnaryCall;
    public createConnection(request: dpm_agent_pb.ConnectionRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.ConnectionResponse) => void): grpc.ClientUnaryCall;
    public compileQuery(request: dpm_agent_pb.Query, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.CompiledQuery) => void): grpc.ClientUnaryCall;
    public compileQuery(request: dpm_agent_pb.Query, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.CompiledQuery) => void): grpc.ClientUnaryCall;
    public compileQuery(request: dpm_agent_pb.Query, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.CompiledQuery) => void): grpc.ClientUnaryCall;
    public executeQuery(request: dpm_agent_pb.Query, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.QueryResult) => void): grpc.ClientUnaryCall;
    public executeQuery(request: dpm_agent_pb.Query, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.QueryResult) => void): grpc.ClientUnaryCall;
    public executeQuery(request: dpm_agent_pb.Query, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.QueryResult) => void): grpc.ClientUnaryCall;
}
