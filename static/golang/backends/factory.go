package backends

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"runtime"

	dpm_agent "github.com/patch-tech/dpm/backends/dpm_agent"
)

// getDpmAuthToken tries to discover the DPM authentication token.
func getDpmAuthToken() (string, error) {
	dpmAuthToken, exists := os.LookupEnv("DPM_AUTH_TOKEN")
	if exists && dpmAuthToken != "" {
		return dpmAuthToken, nil
	}

	var sessionPath string
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return "", fmt.Errorf("failed to get user home directory: %w", err)
	}

	switch os := runtime.GOOS; os {
	case "darwin":
		sessionPath = filepath.Join(homeDir, "Library", "Application Support", "tech.patch.dpm", "session.json")
	case "windows":
		sessionPath = filepath.Join(homeDir, "AppData", "Roaming", "patch", "session.json")
	case "linux":
		sessionPath = filepath.Join(homeDir, ".config", "dpm", "session.json")
	default:
		return "", fmt.Errorf("unsupported operating system: %s", os)
	}

	file, err := os.ReadFile(sessionPath)
	if err != nil {
		return "", fmt.Errorf("error reading session file: %w", err)
	}

	var sessionData map[string]interface{}
	if err := json.Unmarshal(file, &sessionData); err != nil {
		return "", fmt.Errorf("error unmarshalling session data: %w", err)
	}

	accessToken, ok := sessionData["access_token"].(string)
	if !ok {
		return "", fmt.Errorf("access token not found in session data")
	}

	return accessToken, nil
}

// MakeBackend creates a backend instance.
func MakeBackend() (*dpm_agent.DpmAgentServiceClient, error) {
	dpmAuthToken, err := getDpmAuthToken()
	if err != nil {
		return nil, fmt.Errorf("failed to find DPM authentication token: %w", err)
	}

	dpmAgentURL := os.Getenv("DPM_AGENT_URL")
	if dpmAgentURL == "" {
		dpmAgentURL = "https://agent.dpm.sh"
	}

	return dpm_agent.MakeClient(dpmAgentURL, dpmAuthToken)
}
