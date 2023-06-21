// package: dpm_agent
// file: dpm_agent.proto

/* tslint:disable */
/* eslint-disable */

import * as grpc from "@grpc/grpc-js";
import * as dpm_agent_pb from "./dpm_agent_pb";

interface IDpmAgentService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
    createConnection: IDpmAgentService_ICreateConnection;
    executeQuery: IDpmAgentService_IExecuteQuery;
    disconnectConnection: IDpmAgentService_IDisconnectConnection;
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
interface IDpmAgentService_IExecuteQuery extends grpc.MethodDefinition<dpm_agent_pb.Query, dpm_agent_pb.QueryResult> {
    path: "/dpm_agent.DpmAgent/ExecuteQuery";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<dpm_agent_pb.Query>;
    requestDeserialize: grpc.deserialize<dpm_agent_pb.Query>;
    responseSerialize: grpc.serialize<dpm_agent_pb.QueryResult>;
    responseDeserialize: grpc.deserialize<dpm_agent_pb.QueryResult>;
}
interface IDpmAgentService_IDisconnectConnection extends grpc.MethodDefinition<dpm_agent_pb.DisconnectRequest, dpm_agent_pb.DisconnectResponse> {
    path: "/dpm_agent.DpmAgent/DisconnectConnection";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<dpm_agent_pb.DisconnectRequest>;
    requestDeserialize: grpc.deserialize<dpm_agent_pb.DisconnectRequest>;
    responseSerialize: grpc.serialize<dpm_agent_pb.DisconnectResponse>;
    responseDeserialize: grpc.deserialize<dpm_agent_pb.DisconnectResponse>;
}

export const DpmAgentService: IDpmAgentService;

export interface IDpmAgentServer extends grpc.UntypedServiceImplementation {
    createConnection: grpc.handleUnaryCall<dpm_agent_pb.ConnectionRequest, dpm_agent_pb.ConnectionResponse>;
    executeQuery: grpc.handleUnaryCall<dpm_agent_pb.Query, dpm_agent_pb.QueryResult>;
    disconnectConnection: grpc.handleUnaryCall<dpm_agent_pb.DisconnectRequest, dpm_agent_pb.DisconnectResponse>;
}

export interface IDpmAgentClient {
    createConnection(request: dpm_agent_pb.ConnectionRequest, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.ConnectionResponse) => void): grpc.ClientUnaryCall;
    createConnection(request: dpm_agent_pb.ConnectionRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.ConnectionResponse) => void): grpc.ClientUnaryCall;
    createConnection(request: dpm_agent_pb.ConnectionRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.ConnectionResponse) => void): grpc.ClientUnaryCall;
    executeQuery(request: dpm_agent_pb.Query, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.QueryResult) => void): grpc.ClientUnaryCall;
    executeQuery(request: dpm_agent_pb.Query, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.QueryResult) => void): grpc.ClientUnaryCall;
    executeQuery(request: dpm_agent_pb.Query, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.QueryResult) => void): grpc.ClientUnaryCall;
    disconnectConnection(request: dpm_agent_pb.DisconnectRequest, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.DisconnectResponse) => void): grpc.ClientUnaryCall;
    disconnectConnection(request: dpm_agent_pb.DisconnectRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.DisconnectResponse) => void): grpc.ClientUnaryCall;
    disconnectConnection(request: dpm_agent_pb.DisconnectRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.DisconnectResponse) => void): grpc.ClientUnaryCall;
}

export class DpmAgentClient extends grpc.Client implements IDpmAgentClient {
    constructor(address: string, credentials: grpc.ChannelCredentials, options?: Partial<grpc.ClientOptions>);
    public createConnection(request: dpm_agent_pb.ConnectionRequest, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.ConnectionResponse) => void): grpc.ClientUnaryCall;
    public createConnection(request: dpm_agent_pb.ConnectionRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.ConnectionResponse) => void): grpc.ClientUnaryCall;
    public createConnection(request: dpm_agent_pb.ConnectionRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.ConnectionResponse) => void): grpc.ClientUnaryCall;
    public executeQuery(request: dpm_agent_pb.Query, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.QueryResult) => void): grpc.ClientUnaryCall;
    public executeQuery(request: dpm_agent_pb.Query, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.QueryResult) => void): grpc.ClientUnaryCall;
    public executeQuery(request: dpm_agent_pb.Query, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.QueryResult) => void): grpc.ClientUnaryCall;
    public disconnectConnection(request: dpm_agent_pb.DisconnectRequest, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.DisconnectResponse) => void): grpc.ClientUnaryCall;
    public disconnectConnection(request: dpm_agent_pb.DisconnectRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.DisconnectResponse) => void): grpc.ClientUnaryCall;
    public disconnectConnection(request: dpm_agent_pb.DisconnectRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.DisconnectResponse) => void): grpc.ClientUnaryCall;
}
