// package: 
// file: dpm_agent.proto

/* tslint:disable */
/* eslint-disable */

import * as grpc from "@grpc/grpc-js";
import * as dpm_agent_pb from "./dpm_agent_pb";

interface IDpmAgentService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
    connect: IDpmAgentService_IConnect;
    compile: IDpmAgentService_ICompile;
    execute: IDpmAgentService_IExecute;
}

interface IDpmAgentService_IConnect extends grpc.MethodDefinition<dpm_agent_pb.ConnectionRequest, dpm_agent_pb.ConnectionResponse> {
    path: "/DpmAgent/Connect";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<dpm_agent_pb.ConnectionRequest>;
    requestDeserialize: grpc.deserialize<dpm_agent_pb.ConnectionRequest>;
    responseSerialize: grpc.serialize<dpm_agent_pb.ConnectionResponse>;
    responseDeserialize: grpc.deserialize<dpm_agent_pb.ConnectionResponse>;
}
interface IDpmAgentService_ICompile extends grpc.MethodDefinition<dpm_agent_pb.Query, dpm_agent_pb.CompiledQuery> {
    path: "/DpmAgent/Compile";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<dpm_agent_pb.Query>;
    requestDeserialize: grpc.deserialize<dpm_agent_pb.Query>;
    responseSerialize: grpc.serialize<dpm_agent_pb.CompiledQuery>;
    responseDeserialize: grpc.deserialize<dpm_agent_pb.CompiledQuery>;
}
interface IDpmAgentService_IExecute extends grpc.MethodDefinition<dpm_agent_pb.Query, dpm_agent_pb.QueryResult> {
    path: "/DpmAgent/Execute";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<dpm_agent_pb.Query>;
    requestDeserialize: grpc.deserialize<dpm_agent_pb.Query>;
    responseSerialize: grpc.serialize<dpm_agent_pb.QueryResult>;
    responseDeserialize: grpc.deserialize<dpm_agent_pb.QueryResult>;
}

export const DpmAgentService: IDpmAgentService;

export interface IDpmAgentServer extends grpc.UntypedServiceImplementation {
    connect: grpc.handleUnaryCall<dpm_agent_pb.ConnectionRequest, dpm_agent_pb.ConnectionResponse>;
    compile: grpc.handleUnaryCall<dpm_agent_pb.Query, dpm_agent_pb.CompiledQuery>;
    execute: grpc.handleUnaryCall<dpm_agent_pb.Query, dpm_agent_pb.QueryResult>;
}

export interface IDpmAgentClient {
    connect(request: dpm_agent_pb.ConnectionRequest, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.ConnectionResponse) => void): grpc.ClientUnaryCall;
    connect(request: dpm_agent_pb.ConnectionRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.ConnectionResponse) => void): grpc.ClientUnaryCall;
    connect(request: dpm_agent_pb.ConnectionRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.ConnectionResponse) => void): grpc.ClientUnaryCall;
    compile(request: dpm_agent_pb.Query, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.CompiledQuery) => void): grpc.ClientUnaryCall;
    compile(request: dpm_agent_pb.Query, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.CompiledQuery) => void): grpc.ClientUnaryCall;
    compile(request: dpm_agent_pb.Query, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.CompiledQuery) => void): grpc.ClientUnaryCall;
    execute(request: dpm_agent_pb.Query, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.QueryResult) => void): grpc.ClientUnaryCall;
    execute(request: dpm_agent_pb.Query, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.QueryResult) => void): grpc.ClientUnaryCall;
    execute(request: dpm_agent_pb.Query, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.QueryResult) => void): grpc.ClientUnaryCall;
}

export class DpmAgentClient extends grpc.Client implements IDpmAgentClient {
    constructor(address: string, credentials: grpc.ChannelCredentials, options?: Partial<grpc.ClientOptions>);
    public connect(request: dpm_agent_pb.ConnectionRequest, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.ConnectionResponse) => void): grpc.ClientUnaryCall;
    public connect(request: dpm_agent_pb.ConnectionRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.ConnectionResponse) => void): grpc.ClientUnaryCall;
    public connect(request: dpm_agent_pb.ConnectionRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.ConnectionResponse) => void): grpc.ClientUnaryCall;
    public compile(request: dpm_agent_pb.Query, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.CompiledQuery) => void): grpc.ClientUnaryCall;
    public compile(request: dpm_agent_pb.Query, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.CompiledQuery) => void): grpc.ClientUnaryCall;
    public compile(request: dpm_agent_pb.Query, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.CompiledQuery) => void): grpc.ClientUnaryCall;
    public execute(request: dpm_agent_pb.Query, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.QueryResult) => void): grpc.ClientUnaryCall;
    public execute(request: dpm_agent_pb.Query, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.QueryResult) => void): grpc.ClientUnaryCall;
    public execute(request: dpm_agent_pb.Query, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.QueryResult) => void): grpc.ClientUnaryCall;
}
