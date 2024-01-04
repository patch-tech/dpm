package backends

import (
	"context"
	"fmt"
	reflect "reflect"
)

// Direction type, equivalent to Union[Literal["ASC"], Literal["DESC"]] in Python
type Direction string

const (
	ASC  Direction = "ASC"
	DESC Direction = "DESC"
)

// Ordering type, equivalent to Tuple[FieldExpr, Direction] in Python
type Ordering struct {
	Field     interface{}
	Direction Direction
}

type Table struct {
	Backend        Backend
	PackageID      string
	DatasetName    string
	DatasetVersion string
	Source         string
	Name           string
	Fields         []Expr
	FilterExpr     Expr
	Selection      []Expr
	Ordering       []Ordering
	LimitTo        uint64
	NameToField    map[string]Expr
}

func NewTable(
	packageID string,
	datasetName string,
	datasetVersion string,
	name string,
	fields []Expr,
	backend Backend,
	source string,
	filterExpr Expr, // Can be BooleanFieldExpr or UnaryBooleanFieldExpr
	selection []Expr,
	ordering []Ordering,
	limitTo uint64,
) (*Table, error) {
	table := &Table{
		Backend:        backend,
		PackageID:      packageID,
		DatasetName:    datasetName,
		DatasetVersion: datasetVersion,
		Source:         source,
		Name:           name,
		Fields:         fields,
		FilterExpr:     filterExpr,
		Selection:      selection,
		Ordering:       ordering,
		LimitTo:        limitTo,
	}

	// If the backend is not provided, attempt to create one
	if backend == nil {
		var err error
		table.Backend, err = table.GetOrCreateBackend()
		if err != nil {
			return nil, fmt.Errorf("failed to get or create backend: %w", err)
		}
	}

	return table, nil
}

func (t *Table) GetOrCreateBackend() (Backend, error) {
	if t.Backend == nil {
		var err error
		t.Backend, err = MakeBackend()
		if err != nil {
			return nil, err
		}
	}
	return t.Backend, nil
}

func (t *Table) SelectedFieldExpr(selector interface{}) (Expr, error) {
	switch sel := selector.(type) {
	case *FieldExpr:
		return sel, nil
	case *Field:
		return sel, nil
	case *AggregateFieldExpr:
		return sel, nil
	case *DerivedField:
		return sel, nil
	case *BooleanFieldExpr:
		return sel, nil
	case *UnaryBooleanFieldExpr:
		return sel, nil
	case *LiteralField:
		return sel, nil
	case *DateField:
		return sel, nil
	case *TimeField:
		return sel, nil
	case *DateTimeField:
		return sel, nil
	case *ArrayField:
		return sel, nil
	case string:
		// If selector is a string, look it up in the name_to_field map
		fieldExpr, ok := t.NameToField[sel]
		if !ok {
			// If the field is not found, return an error
			return nil, fmt.Errorf("unknown field selector \"%s\"", sel)
		}
		return fieldExpr.(*FieldExpr), nil
	default:
		// If the selector is neither a string nor a FieldExpr, return an error
		return nil, fmt.Errorf("invalid selector type")
	}
}

func getAliasFromExpr(expr interface{}) (string, bool) {
	val := reflect.ValueOf(expr)

	// Check if the value is valid and is a struct or a pointer to a struct
	if !val.IsValid() || (val.Kind() != reflect.Struct && val.Kind() != reflect.Ptr) {
		return "", false
	}

	// If it's a pointer, we get the element it points to
	if val.Kind() == reflect.Ptr {
		val = val.Elem()
	}

	// Check if the struct has a field named "Alias"
	aliasField := val.FieldByName("Alias")
	if aliasField.IsValid() && aliasField.Kind() == reflect.Ptr && !aliasField.IsNil() {
		// Assuming Alias is of type *string
		return *aliasField.Interface().(*string), true
	}

	return "", false
}

func (t *Table) OrderByExpr(selector interface{}) (Expr, error) {
	fieldExpr, err := t.SelectedFieldExpr(selector)
	if err == nil {
		return fieldExpr, nil
	}

	selStr, isStr := selector.(string)
	if isStr && t.Selection != nil {
		for _, expr := range t.Selection {
			if alias, _ := getAliasFromExpr(expr); alias == selStr {
				return expr, nil
			}
		}
	}

	return nil, fmt.Errorf("unknown field selector \"%v\"", selector)
}

func (t *Table) Filter(expr Expr) *Table {
	// Create a new Table instance with the same values as the current one
	newTable := *t
	// Update the FilterExpr field with the new expression
	newTable.FilterExpr = expr
	return &newTable
}

func (t *Table) Select(selections ...interface{}) *Table {
	newTable := *t // Create a new Table instance with the same values as the current one

	// Initialize a new slice for selected field expressions
	var selectExprs []Expr
	for _, sel := range selections {
		fieldExpr, err := t.SelectedFieldExpr(sel)
		if err == nil {
			exprInterface := Expr(fieldExpr)                 // Convert to Expr interface
			selectExprs = append(selectExprs, exprInterface) // Append as *Expr
		}
	}
	// Update the Selection field with the new selection expressions
	newTable.Selection = selectExprs
	return &newTable
}

func (t *Table) OrderBy(orderings ...Ordering) *Table {
	newTable := *t // Create a new Table instance with the same values as the current one

	// Initialize a new slice for ordering
	var newOrdering []Ordering
	for _, ord := range orderings {
		fieldExpr, err := t.OrderByExpr(ord.Field)
		if err == nil {
			// Append the ordering with the resolved field expression
			newOrdering = append(newOrdering, Ordering{
				Field:     fieldExpr,
				Direction: ord.Direction,
			})
		}
	}

	// Update the Ordering field with the new ordering expressions
	newTable.Ordering = newOrdering
	return &newTable
}

func (t *Table) Limit(n uint64) *Table {
	newTable := *t       // Create a new Table instance with the same values as the current one
	newTable.LimitTo = n // Set the limit
	return &newTable     // Return a pointer to the new Table instance
}

// Compile compiles the table expression into a query string on its execution backend.
// For example, it returns a Snowsql string for a table expression with a Snowflake execution backend.
func (t *Table) Compile() (string, error) {
	backend, err := t.GetOrCreateBackend()
	if err != nil {
		return "", fmt.Errorf("error getting or creating backend: %w", err)
	}
	return backend.Compile(context.Background(), t)
}

// Execute executes the table expression on its execution backend and resolves to the results.
func (t *Table) Execute() ([]map[string]interface{}, error) {
	backend, err := t.GetOrCreateBackend()
	if err != nil {
		return nil, fmt.Errorf("error getting or creating backend: %w", err)
	}
	return backend.Execute(context.Background(), t)
}
