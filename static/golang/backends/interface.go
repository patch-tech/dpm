package backends

import "context"

type Backend interface {
	Compile(ctx context.Context, query string) (string, error)
	Execute(ctx context.Context, query string) ([]map[string]interface{}, error)
}
