using DpmAgent;
using Grpc.Net.Client;
using System.Collections.Specialized;
using System.Reflection.Metadata.Ecma335;
using System.Text;
using System.Threading.Channels;

namespace Dpm
{
    public class DpmAgentClient : IDisposable {

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

            public DpmAgentClient Build()
            {
                return new DpmAgentClient(endpoint, authToken);
            }

        }

        public static Builder NewBuilder()
        {
            return new Builder();
        }

        GrpcChannel channel;
        DpmAgent.DpmAgent.DpmAgentClient client;
        String? authToken;

        public DpmAgentClient(String agentUrl, String? authToken) {
            this.channel = GrpcChannel.ForAddress(agentUrl);
            this.client = new DpmAgent.DpmAgent.DpmAgentClient(channel);
            this.authToken = authToken;
        }

        public void Dispose()
        {
            this.channel.Dispose();
        }
        public QueryResult ExecuteQuery(DpmAgent.Query request)
        {
            return client.ExecuteQuery(request, headers());
        }

        public Grpc.Core.AsyncUnaryCall<QueryResult> ExecuteQueryAsync(DpmAgent.Query request)
        {
            return client.ExecuteQueryAsync(request, headers());
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
