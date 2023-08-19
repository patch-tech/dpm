// package: dpm_agent
// file: dpm_agent.proto

/* tslint:disable */
/* eslint-disable */

import * as grpc from "@grpc/grpc-js";
import * as dpm_agent_pb from "./dpm_agent_pb";

interface IDpmAgentService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
    executeQuery: IDpmAgentService_IExecuteQuery;
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
    executeQuery: grpc.handleUnaryCall<dpm_agent_pb.Query, dpm_agent_pb.QueryResult>;
}

export interface IDpmAgentClient {
    executeQuery(request: dpm_agent_pb.Query, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.QueryResult) => void): grpc.ClientUnaryCall;
    executeQuery(request: dpm_agent_pb.Query, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.QueryResult) => void): grpc.ClientUnaryCall;
    executeQuery(request: dpm_agent_pb.Query, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.QueryResult) => void): grpc.ClientUnaryCall;
}

export class DpmAgentClient extends grpc.Client implements IDpmAgentClient {
    constructor(address: string, credentials: grpc.ChannelCredentials, options?: Partial<grpc.ClientOptions>);
    public executeQuery(request: dpm_agent_pb.Query, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.QueryResult) => void): grpc.ClientUnaryCall;
    public executeQuery(request: dpm_agent_pb.Query, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.QueryResult) => void): grpc.ClientUnaryCall;
    public executeQuery(request: dpm_agent_pb.Query, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: dpm_agent_pb.QueryResult) => void): grpc.ClientUnaryCall;
}
