using System.Collections.Concurrent;

namespace Dpm
{
  /// <summary>
  /// A Singleton class that creates DpmAgentClient instances, one per service
  /// address.
  /// </summary>
  public class DpmAgentClientFactory
  {
    readonly ConcurrentDictionary<string, DpmAgentClient> agentClients = new();

    // Singleton.
    private static readonly Lazy<DpmAgentClientFactory> lazy = new(() => new DpmAgentClientFactory());

    private DpmAgentClientFactory() { }

    private static DpmAgentClientFactory Instance { get { return lazy.Value; } }

    public static DpmAgentClient MakeClient(string dpmAgentServiceAddress, string dpmAuthToken)
    {
      if (!Instance.agentClients.ContainsKey(dpmAgentServiceAddress))
      {
        Instance.agentClients[dpmAgentServiceAddress] = new DpmAgentClient(dpmAgentServiceAddress, dpmAuthToken);
      }
      return Instance.agentClients[dpmAgentServiceAddress];
    }

    public static DpmAgentClient MakeClient()
    {
      string dpmAgentServiceAddress = Env.GetDpmAgentServiceAddress();
      string dpmAuthToken = Env.GetDpmAuthToken() ?? throw new Exception("Failed to find DPM authentication token. Please run `dpm login`");
      return MakeClient(dpmAgentServiceAddress, dpmAuthToken); ;
    }
  }
}