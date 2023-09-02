using DpmAgent;
using Grpc.Net.Client;
using System.Collections.Specialized;
using System.Reflection.Metadata.Ecma335;
using System.Text;
using System.Threading.Channels;

namespace Dpm
{
    public class DatasetClient : IDisposable {

        public class Builder
        {

            String endpoint;
            String? authToken = null;
            DpmAgent.Query.Types.Id datasetId;

            public Builder()
            {
                endpoint = "https://agent.dpm.sh";
                datasetId = new DpmAgent.Query.Types.Id();
            }

            public Builder SetEndpoint(String endpoint)
            {
                this.endpoint = endpoint;
                return this;
            }

            public Builder SetPackageId(String packageId)
            {
                datasetId = new DpmAgent.Query.Types.Id
                {
                    PackageId = packageId
                };
                return this;
            }

            public Builder SetSourceId(String sourceId)
            {
                datasetId = new DpmAgent.Query.Types.Id
                {
                    SourceId = sourceId
                };
                return this;
            }

            public Builder SetAuthToken(String token)
            {
                authToken = token;
                return this;
            }

            public DatasetClient Build()
            {
                return new DatasetClient(endpoint, datasetId, authToken);
            }

        }

        public static Builder NewBuilder()
        {
            return new Builder();
        }

        GrpcChannel channel;
        DpmAgent.DpmAgent.DpmAgentClient client;
        DpmAgent.Query.Types.Id datasetId;
        DpmAgent.ClientVersion clientVersion;
        String? authToken;

        public DatasetClient(String agentUrl, DpmAgent.Query.Types.Id datasetId, String? authToken) {
            this.channel = GrpcChannel.ForAddress(agentUrl);
            this.client = new DpmAgent.DpmAgent.DpmAgentClient(channel);
            this.datasetId = datasetId;
            this.authToken = authToken;
            this.clientVersion = new ClientVersion
            {
                Client = ClientVersion.Types.Client.Csharp,
                DatasetVersion = "0.1.0",
                CodeVersion = "0.1.0"
            };
        }

        public void Dispose()
        {
            this.channel.Dispose();
        }
        public QueryResult ExecuteQuery(DpmAgent.Query request)
        {
            return client.ExecuteQuery(decorateQuery(request), headers());
        }

        public Grpc.Core.AsyncUnaryCall<QueryResult> ExecuteQueryAsync(DpmAgent.Query request)
        {
            return client.ExecuteQueryAsync(request, headers());
        }

        DpmAgent.Query decorateQuery(DpmAgent.Query request)
        {
            request.ClientVersion = clientVersion;
            request.Id = datasetId;
            return request;
        }

        Grpc.Core.Metadata headers()
        {
            var md = new Grpc.Core.Metadata();
            if (authToken != null)
            {
                md.Add(new Grpc.Core.Metadata.Entry(
                    "dpm-auth-token",
                    Encoding.UTF8.GetBytes(authToken)));
            }
            return md;
        }
    }
}
