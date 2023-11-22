package backends

const CODE_VERSION = "0.1.0"

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
	Fields         []*Expr
	FilterExpr     interface{} // Can be BooleanFieldExpr or UnaryBooleanFieldExpr
	Selection      []*Expr
	Ordering       []Ordering
	LimitTo        uint64
	NameToField    map[string]*Expr
}
