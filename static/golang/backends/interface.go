package backends

import "context"

type Backend interface {
	Compile(ctx context.Context, query *Table) (string, error)
	Execute(ctx context.Context, query *Table) ([]map[string]interface{}, error)
}
