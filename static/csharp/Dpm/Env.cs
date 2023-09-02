using System.Data;
using System.Runtime.InteropServices;
using System.Text.Json;
using System.Text.Json.Nodes;

namespace Dpm
{
  public static class Env
  {
    public static string GetDpmAgentServiceAddress()
    {
      return Environment.GetEnvironmentVariable("DPM_AGENT_URL") ?? "https://agent.dpm.sh";
    }


    record Session(string access_token, string token_type, string? expires_in, string scope);

    public static string? GetDpmAuthToken()
    {
      var dpmAuthToken = Environment.GetEnvironmentVariable("DPM_AUTH_TOKEN");
      if (dpmAuthToken != null)
      {
        return dpmAuthToken;
      }

      Console.WriteLine("Discovering DPM Auth Token from session data.");
      var rootDir = Environment.GetFolderPath(Environment.SpecialFolder.UserProfile);
      var sessionPath = "";
      if (RuntimeInformation.IsOSPlatform(OSPlatform.Linux))
      {
        sessionPath = Path.Join(rootDir, ".config", "dpm", "session.json");
      }
      else if (RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
      {
        sessionPath = Path.Join(rootDir, "AppData", "Roaming", "patch", "session.json");
      }
      else if (RuntimeInformation.IsOSPlatform(OSPlatform.OSX))
      {
        sessionPath = Path.Join(rootDir, "Library", "Application Support", "tech.patch.dpm", "session.json");
      }

      try
      {
        using StreamReader r = new(sessionPath);
        var sessionString = r.ReadToEnd();
        var sessionData = JsonSerializer.Deserialize<Session>(sessionString);
        return sessionData?.access_token;
      }
      catch (Exception e)
      {
        Console.Error.WriteLine("Error getting access token from project directory:", e.Message);
      }

      return null;
    }
  }
}