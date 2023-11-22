package models

import "context"

type Backend interface {
	Compile(ctx context.Context, query string) (string, error)
	Execute(ctx context.Context, query string) ([]map[string]interface{}, error)
}

// Direction type, equivalent to Union[Literal["ASC"], Literal["DESC"]] in Python
type Direction string

const (
	ASC  Direction = "ASC"
	DESC Direction = "DESC"
)

// Ordering type, equivalent to Tuple[FieldExpr, Direction] in Python
type Ordering struct {
	Field     *FieldExpr
	Direction Direction
}

// Table struct, equivalent to the Table class in Python
type Table struct {
	Backend        Backend // Assuming Backend is an interface or struct
	PackageID      string
	DatasetName    string
	DatasetVersion string
	Source         string
	Name           string
	Fields         []*FieldExpr
	FilterExpr     interface{} // Can be BooleanFieldExpr or UnaryBooleanFieldExpr
	Selection      []*FieldExpr
	Ordering       []Ordering
	LimitTo        int
	NameToField    map[string]*FieldExpr
}

// NewTable function to create a new Table instance
func NewTable(packageID, datasetName, datasetVersion, name string, fields []*FieldExpr, backend Backend, source string, filterExpr interface{}, selection []*FieldExpr, ordering []Ordering, limitTo int) *Table {
	t := &Table{
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
	t.NameToField = make(map[string]*FieldExpr)
	for _, field := range fields {
		t.NameToField[field.Name] = field
	}
	//t.getOrMakeBackend() // This needs to be implemented
	return t
}

/* func (t *Table) getOrMakeBackend() (backends.Backend, error) {
	if t.Backend == nil {
		backend, err := backends.MakeBackend()
		if err != nil {
			return nil, err
		}
		t.Backend = backend
	}
	return t.Backend
} */

// Copy method for Table
func (t *Table) Copy(name string, fields []*FieldExpr, filterExpr interface{}, selection []*FieldExpr, ordering []Ordering, limitTo int) *Table {
	if name == "" {
		name = t.Name
	}
	if fields == nil {
		fields = t.Fields
	}
	if filterExpr == nil {
		filterExpr = t.FilterExpr
	}
	if selection == nil {
		selection = t.Selection
	}
	if ordering == nil {
		ordering = t.Ordering
	}
	if limitTo == 0 {
		limitTo = t.LimitTo
	}
	return NewTable(t.PackageID, t.DatasetName, t.DatasetVersion, name, fields, t.Backend, t.Source, filterExpr, selection, ordering, limitTo)
}
