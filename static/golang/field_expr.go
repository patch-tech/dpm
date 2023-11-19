package main

import "fmt"

type Scalar interface{} // can be string, int, float64, bool

type UnaryOperator string

const (
	IsNull    UnaryOperator = "isNull"
	IsNotNull UnaryOperator = "isNotNull"
)

type BooleanOperator string

const (
	Eq  BooleanOperator = "eq"
	Neq BooleanOperator = "neq"
	Gt  BooleanOperator = "gt"
	Gte BooleanOperator = "gte"
	Lt  BooleanOperator = "lt"
	Lte BooleanOperator = "lte"
	And BooleanOperator = "and"
	Or  BooleanOperator = "or"
)

type ArithmeticOperator string

const (
	Add      ArithmeticOperator = "+"
	Subtract ArithmeticOperator = "-"
	Multiply ArithmeticOperator = "*"
	Divide   ArithmeticOperator = "/"
)

type AggregateOperator string

const (
	Min           AggregateOperator = "min"
	Max           AggregateOperator = "max"
	Sum           AggregateOperator = "sum"
	Count         AggregateOperator = "count"
	CountDistinct AggregateOperator = "countDistinct"
)

type DateOperator string

const (
	Years  DateOperator = "years"
	Months DateOperator = "months"
	Days   DateOperator = "days"
)

type TimeOperator string

const (
	Hour        TimeOperator = "hour"
	Minute      TimeOperator = "minute"
	Second      TimeOperator = "second"
	Millisecond TimeOperator = "millisecond"
)

type ProjectionOperator string

// DateOperator and TimeOperator values can be used here

type DateGranularity string

const (
	YearsGranularity  DateGranularity = "years"
	MonthsGranularity DateGranularity = "months"
	WeeksGranularity  DateGranularity = "weeks"
	DaysGranularity   DateGranularity = "days"
)

type TimeGranularity string

const (
	HoursGranularity        TimeGranularity = "hours"
	MinutesGranularity      TimeGranularity = "minutes"
	SecondsGranularity      TimeGranularity = "seconds"
	MillisecondsGranularity TimeGranularity = "milliseconds"
)

type DateTimeGranularity string

const (
	DateTimeYearsGranularity        DateTimeGranularity = "years"
	DateTimeMonthsGranularity       DateTimeGranularity = "months"
	DateTimeWeeksGranularity        DateTimeGranularity = "weeks"
	DateTimeDaysGranularity         DateTimeGranularity = "days"
	DateTimeHoursGranularity        DateTimeGranularity = "hours"
	DateTimeMinutesGranularity      DateTimeGranularity = "minutes"
	DateTimeSecondsGranularity      DateTimeGranularity = "seconds"
	DateTimeMillisecondsGranularity DateTimeGranularity = "milliseconds"
)

type Operator string

// UnaryOperator, BooleanOperator, ArithmeticOperator, AggregateOperator values can be used here

type Expr interface {
	Operator() Operator
	Operands() []Expr
}

type FieldExpr struct {
	Expr
	Name  string
	Alias *string
}

// NewFieldExpr creates a new FieldExpr with the provided name and optional alias.
// If no alias is provided, alias is set to nil.
func NewFieldExpr(name string, alias ...string) *FieldExpr {
	var aliasPtr *string
	if len(alias) > 0 {
		aliasPtr = &alias[0]
	}

	return &FieldExpr{Name: name, Alias: aliasPtr}
}

func (f *FieldExpr) ToString() string {
	return f.Name
}

// BooleanFieldExpr represents a binary boolean expression.
// It contains two field expressions and a boolean operator applied to them.
type BooleanFieldExpr struct {
	Expr
	FieldExpr
	Field Expr
	Op    BooleanOperator
	Other Expr
}

// NewBooleanFieldExpr creates a new BooleanFieldExpr with the given fields and operator.
// It represents binary boolean operations like equality, inequality, and logical operators.
func NewBooleanFieldExpr(field, other Expr, op BooleanOperator) *BooleanFieldExpr {
	var fieldExpr FieldExpr

	switch f := field.(type) {
	case *DateField:
		// Accessing the embedded FieldExpr from DateField
		fieldExpr = f.FieldExpr
	case *TimeField:
		// Similarly for TimeField or other types that embed FieldExpr
		fieldExpr = f.FieldExpr
	case *DateTimeField:
		fieldExpr = f.FieldExpr
	default:
		// Handle the case where the type directly implements FieldExpr
		var ok bool
		if fieldExpr, ok = field.(FieldExpr); !ok {
			panic(fmt.Sprintf("field of type %T does not implement FieldExpr", field))
		}
	}
	return &BooleanFieldExpr{
		FieldExpr: fieldExpr,
		Field:     field,
		Op:        op,
		Other:     other,
	}
}

// Operator returns the operator of this binary boolean expression.
func (b *BooleanFieldExpr) Operator() Operator {
	return Operator(b.Op)
}

// Operands returns the operands of this binary boolean expression.
// It returns both the field expressions involved in the operation.
func (b *BooleanFieldExpr) Operands() []Expr {
	return []Expr{b.Field, b.Other}
}

// And creates a new BooleanFieldExpr representing the logical AND of this expression and another field expression.
// It is used for combining two boolean expressions with an AND operation.
func (b *BooleanFieldExpr) And(that Expr) *BooleanFieldExpr {
	return &BooleanFieldExpr{Field: b.Field, Op: "and", Other: that}
}

// Or creates a new BooleanFieldExpr representing the logical OR of this expression and another field expression.
// It is used for combining two boolean expressions with an OR operation.
func (b *BooleanFieldExpr) Or(that Expr) *BooleanFieldExpr {
	return &BooleanFieldExpr{Field: b.Field, Op: "or", Other: that}
}

// UnaryBooleanFieldExpr represents a unary boolean expression.
// It contains a field and a unary operator applied to that field.
type UnaryBooleanFieldExpr struct {
	Expr
	FieldExpr
	Field FieldExpr
	Op    UnaryOperator
}

// NewUnaryBooleanFieldExpr creates a new UnaryBooleanFieldExpr with the given field and operator.
func NewUnaryBooleanFieldExpr(field FieldExpr, op UnaryOperator) *UnaryBooleanFieldExpr {
	return &UnaryBooleanFieldExpr{
		FieldExpr: FieldExpr{Name: fmt.Sprintf("(%s(%s))", op, field.Name)},
		Field:     field,
		Op:        op,
	}
}

// Operator returns the operator of this unary boolean expression.
func (u *UnaryBooleanFieldExpr) Operator() Operator {
	return Operator(u.Op)
}

// Operands returns the operands of this unary boolean expression.
// Since it's unary, it returns only the field it operates on.
func (u *UnaryBooleanFieldExpr) Operands() []Expr {
	return []Expr{&u.Field}
}

// And creates a new BooleanFieldExpr representing the logical AND of this expression and another field expression.
func (u *UnaryBooleanFieldExpr) And(that FieldExpr) *BooleanFieldExpr {
	return &BooleanFieldExpr{Field: u.Field, Op: "and", Other: that}
}

// Or creates a new BooleanFieldExpr representing the logical OR of this expression and another field expression.
func (u *UnaryBooleanFieldExpr) Or(that FieldExpr) *BooleanFieldExpr {
	return &BooleanFieldExpr{Field: u.Field, Op: "or", Other: that}
}

// AggregateFieldExpr represents an aggregation operation applied to a field expression.
// It contains a field expression and an aggregate operator like sum, min, max, etc.
type AggregateFieldExpr struct {
	Expr
	FieldExpr
	Field FieldExpr
	Op    AggregateOperator
}

// NewAggregateFieldExpr creates a new AggregateFieldExpr with the given field and aggregation operator.
// This is used to represent aggregate operations like sum, count, min, and max on a field.
func NewAggregateFieldExpr(field FieldExpr, op AggregateOperator) *AggregateFieldExpr {
	return &AggregateFieldExpr{
		FieldExpr: FieldExpr{Name: fmt.Sprintf("(%s(%s))", op, field.Name)},
		Field:     field,
		Op:        op,
	}
}

// WithAlias sets an alias for the aggregate expression.
// This is useful when the expression is used in a select and needs to be referenced in an order by.
func (a *AggregateFieldExpr) WithAlias(alias string) *AggregateFieldExpr {
	copy := *a
	copy.Alias = &alias
	return &copy
}

// Operator returns the operator of this aggregate expression.
func (a *AggregateFieldExpr) Operator() Operator {
	return Operator(a.Op)
}

// Operands returns the operands of this aggregate expression.
// Since it's an aggregation, it returns only the field it operates on.
func (a *AggregateFieldExpr) Operands() []Expr {
	return []Expr{&a.Field}
}
