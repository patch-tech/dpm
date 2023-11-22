package models

import (
	"fmt"
	"log"
	"time"
)

// Field represents a field in a database table.
// It embeds FieldExpr, allowing it to inherit or override methods and properties of FieldExpr.
type Field struct {
	FieldExpr
}

type StringExpr struct {
	Expr
	Value string
}

func (s StringExpr) Operands() []Expr {
	return []Expr{s}
}

// NewField creates and returns a new Field instance.
// It takes the name of the field as a parameter and initializes a Field object.
func NewField(name string) *Field {
	return &Field{
		FieldExpr: FieldExpr{Name: name},
	}
}

// WithAlias returns a new field object with specified alias.
func (f *Field) WithAlias(alias string) *FieldExpr {
	new_field := *f          // Creates a copy of the field
	new_field.Alias = &alias // Sets the alias of the field
	return &new_field.FieldExpr
}

// Operator returns the operator for the field.
func (f *Field) Operator() Operator {
	return "ident"
}

// Operands returns the operands for the field.
func (f *Field) Operands() []Expr {
	return []Expr{StringExpr{Value: f.Name}}
}

// Max creates and returns an AggregateFieldExpr representing a MAX aggregation.
// This function uses the NewAggregateFieldExpr constructor to create the expression.
func (f *Field) Max() *AggregateFieldExpr {
	return NewAggregateFieldExpr(f.FieldExpr, Max) // Creates a new AggregateFieldExpr with MAX operation
}

// AvgDistinct creates and returns an AggregateFieldExpr representing an AVG_DISTINCT aggregation.
func (f *Field) AvgDistinct() *AggregateFieldExpr {
	// Use the Field instance itself as the operand
	return NewAggregateFieldExpr(f.FieldExpr, "avgDistinct")
}

// Sum creates and returns an AggregateFieldExpr representing a SUM aggregation.
func (f *Field) Sum() *AggregateFieldExpr {
	return NewAggregateFieldExpr(f.FieldExpr, Sum)
}

// DerivedField represents a field derived from another field by applying a projection operator.
type DerivedField struct {
	Field           // Embedding the Field struct for reusability
	Op       string // Projection operator applied to derive this field
	Original *Field // The original field from which this is derived
}

// NewDerivedField creates and returns a new DerivedField instance.
func NewDerivedField(field *Field, op string) *DerivedField {
	derivedFieldName := "(" + op + "(" + field.Name + "))"
	return &DerivedField{
		Field:    Field{FieldExpr: FieldExpr{Name: derivedFieldName}},
		Op:       op,
		Original: field,
	}
}

// Operator returns the operator for the derived field.
func (d *DerivedField) Operator() Operator {
	return Operator(d.Op)
}

// Operands returns the operands for the derived field.
func (d *DerivedField) Operands() []Expr {
	return []Expr{d.Original}
}

// WithAlias sets an alias for the derived field and returns the field instance.
func (d *DerivedField) WithAlias(alias string) *DerivedField {
	newDerived := *d          // Creates a copy of the derived field
	newDerived.Alias = &alias // Sets the alias of the derived field
	return &newDerived
}

// LiteralField represents a literal field value in an expression.
type LiteralField struct {
	FieldExpr // Embedding FieldExpr for compatibility with field-like operations
	Value     interface{}
}

// NewLiteralField creates and returns a new LiteralField instance.
func NewLiteralField(value interface{}) *LiteralField {
	return &LiteralField{
		FieldExpr: FieldExpr{Name: fmt.Sprintf("lit(%v)", value)},
		Value:     value,
	}
}

// Operator returns the operator for the literal field, which is "ident".
func (l *LiteralField) Operator() Operator {
	return "ident"
}

// Operands returns the operands for the literal field.
func (l *LiteralField) Operands() []Expr {
	// Check if Value implements Expr interface
	if expr, ok := l.Value.(Expr); ok {
		return []Expr{expr}
	}

	// Handle the case where Value is a slice of Expr
	if valueSlice, ok := l.Value.([]Expr); ok {
		return valueSlice
	}

	// If Value is not an Expr and not a slice of Expr, wrap it in a custom Expr
	return []Expr{&SimpleExpr{Value: l.Value}}
}

// SimpleExpr is a basic implementation of the Expr interface for non-Expr values.
type SimpleExpr struct {
	Value interface{}
}

// Operator returns a default operator for SimpleExpr.
func (s *SimpleExpr) Operator() Operator {
	return "value"
}

// Operands returns the value wrapped in a slice.
func (s *SimpleExpr) Operands() []Expr {
	return []Expr{s}
}

// Disallowed methods that raise errors in the Python version.

// Max disallows calling max on a literal field.
func (l *LiteralField) Max() error {
	return fmt.Errorf("cannot call max on literal field")
}

func (l *LiteralField) Min() error {
	return fmt.Errorf("cannot call min on literal field")
}

func (l *LiteralField) Sum() error {
	return fmt.Errorf("cannot call sum on literal field")
}

func (l *LiteralField) Count() error {
	return fmt.Errorf("cannot call count on literal field")
}

func (l *LiteralField) CountDistinct() error {
	return fmt.Errorf("cannot call count_distinct on literal field")
}

func (l *LiteralField) Avg() error {
	return fmt.Errorf("cannot call avh on literal field")
}

func (l *LiteralField) AvgDistinct() error {
	return fmt.Errorf("cannot call avg_distinct on literal field")
}

// Helper function to add duration based on granularity
func AddDateDuration(t time.Time, amount int, granularity DateGranularity) time.Time {
	switch granularity {
	case DaysGranularity:
		return t.AddDate(0, 0, amount)
	case WeeksGranularity:
		return t.AddDate(0, 0, amount*7) // 7 days in a week
	case MonthsGranularity:
		return t.AddDate(0, amount, 0)
	case YearsGranularity:
		return t.AddDate(amount, 0, 0)
	default:
		// Handle default case or error
		return t
	}
}

// DateField represents a date field in a database table, extending the Field struct.
type DateField struct {
	Field
}

func (s *DateField) Operator() Operator {
	return "value"
}

// Operands returns the value wrapped in a slice.
func (s *DateField) Operands() []Expr {
	return []Expr{s}
}

// NewDateField creates and returns a new DateField instance.
func NewDateField(name string) *DateField {
	return &DateField{
		Field: Field{FieldExpr: FieldExpr{Name: name}},
	}
}

// Month returns a DerivedField representing the month of the DateField.
func (d *DateField) Month() *DerivedField {
	return NewDerivedField(&d.Field, "month")
}

// Day returns a DerivedField representing the day of the DateField.
func (d *DateField) Day() *DerivedField {
	return NewDerivedField(&d.Field, "day")
}

// Year returns a DerivedField representing the year of the DateField.
func (d *DateField) Year() *DerivedField {
	return NewDerivedField(&d.Field, "year")
}

// Before returns a BooleanFieldExpr checking if the DateField is before the provided date.
func (df *DateField) Before(d time.Time) *BooleanFieldExpr {
	literalField := NewLiteralField(d.Format(time.DateOnly))
	return NewBooleanFieldExpr(df, literalField, Lt)
}

// After returns a BooleanFieldExpr checking if the DateField is after the provided date.
func (df *DateField) After(d time.Time) *BooleanFieldExpr {
	literalField := NewLiteralField(d.Format(time.DateOnly))
	return NewBooleanFieldExpr(df, literalField, Gt)
}

func (df *DateField) LessThan(d time.Time) *BooleanFieldExpr {
	return df.Before(d)
}

func (df *DateField) GreaterThan(d time.Time) *BooleanFieldExpr {
	return df.After(d)
}

func (df *DateField) LessThanOrEqual(d time.Time) *BooleanFieldExpr {
	literalField := NewLiteralField(d.Format(time.DateOnly))
	return NewBooleanFieldExpr(df, literalField, Lte)
}

func (df *DateField) GreaterThanOrEqual(d time.Time) *BooleanFieldExpr {
	literalField := NewLiteralField(d.Format(time.DateOnly))
	return NewBooleanFieldExpr(df, literalField, Gte)
}

func (df *DateField) Equal(d time.Time) *BooleanFieldExpr {
	literalField := NewLiteralField(d.Format(time.DateOnly))
	return NewBooleanFieldExpr(df, literalField, Eq)
}

func (df *DateField) NotEquals(d time.Time) *BooleanFieldExpr {
	literalField := NewLiteralField(d.Format(time.DateOnly))
	return NewBooleanFieldExpr(df, literalField, Neq)
}

func (df *DateField) InPast(olderThan int, newerThan int, granularity DateGranularity) *BooleanFieldExpr {
	// Swap if older_than is greater than newer_than
	if olderThan > newerThan {
		olderThan, newerThan = newerThan, olderThan
	}

	today := time.Now().UTC()
	upper := AddDateDuration(today, -olderThan, granularity)
	lower := AddDateDuration(today, -newerThan, granularity)

	// Assuming you have appropriate methods for comparison
	return df.GreaterThanOrEqual(lower).And(df.LessThanOrEqual(upper))
}

func AddTimeDuration(t time.Time, delta int, granularity TimeGranularity) time.Time {
	// Add the duration based on the granularity
	var addedTime time.Time
	switch granularity {
	case HoursGranularity:
		addedTime = t.Add(time.Duration(delta) * time.Hour)
	case MinutesGranularity:
		addedTime = t.Add(time.Duration(delta) * time.Minute)
	case SecondsGranularity:
		addedTime = t.Add(time.Duration(delta) * time.Second)
	case MillisecondsGranularity:
		addedTime = t.Add(time.Duration(delta) * time.Millisecond)
	default:
		return t
	}

	// Clamping logic
	if addedTime.Day() != t.Day() {
		if delta < 0 {
			return time.Date(t.Year(), t.Month(), t.Day(), 0, 0, 0, 0, t.Location())
		} else {
			return time.Date(t.Year(), t.Month(), t.Day(), 23, 59, 59, 999999000, t.Location())
		}
	}

	return addedTime
}

type TimeField struct {
	Field // Embedding the Field struct for reusability
}

func NewTimeField(name string) *TimeField {
	return &TimeField{
		Field: Field{FieldExpr: FieldExpr{Name: name}},
	}
}

func (d *TimeField) Hour() *DerivedField {
	return NewDerivedField(&d.Field, "hour")
}

func (d *TimeField) Minute() *DerivedField {
	return NewDerivedField(&d.Field, "minute")
}

func (d *TimeField) Second() *DerivedField {
	return NewDerivedField(&d.Field, "second")
}

func (df *TimeField) Before(d time.Time) *BooleanFieldExpr {
	literalField := NewLiteralField(d.Format(time.TimeOnly))
	return NewBooleanFieldExpr(df, literalField, Lt)
}

func (df *TimeField) After(d time.Time) *BooleanFieldExpr {
	literalField := NewLiteralField(d.Format(time.TimeOnly))
	return NewBooleanFieldExpr(df, literalField, Gt)
}

func (df *TimeField) LessThan(d time.Time) *BooleanFieldExpr {
	return df.Before(d)
}

func (df *TimeField) GreaterThan(d time.Time) *BooleanFieldExpr {
	return df.After(d)
}

func (df *TimeField) LessThanOrEqual(d time.Time) *BooleanFieldExpr {
	literalField := NewLiteralField(d.Format(time.TimeOnly))
	return NewBooleanFieldExpr(df, literalField, Lte)
}

func (df *TimeField) GreaterThanOrEqual(d time.Time) *BooleanFieldExpr {
	literalField := NewLiteralField(d.Format(time.TimeOnly))
	return NewBooleanFieldExpr(df, literalField, Gte)
}

func (df *TimeField) Equal(d time.Time) *BooleanFieldExpr {
	literalField := NewLiteralField(d.Format(time.TimeOnly))
	return NewBooleanFieldExpr(df, literalField, Eq)
}

func (df *TimeField) NotEquals(d time.Time) *BooleanFieldExpr {
	literalField := NewLiteralField(d.Format(time.TimeOnly))
	return NewBooleanFieldExpr(df, literalField, Neq)
}

func (df *TimeField) InPast(olderThan int, newerThan int, granularity TimeGranularity) *BooleanFieldExpr {
	// Swap if older_than is greater than newer_than
	if olderThan > newerThan {
		olderThan, newerThan = newerThan, olderThan
	}

	today := time.Now().UTC()
	upper := AddTimeDuration(today, -olderThan, granularity)
	lower := AddTimeDuration(today, -newerThan, granularity)

	// Assuming you have appropriate methods for comparison
	return df.GreaterThanOrEqual(lower).And(df.LessThanOrEqual(upper))
}

type DateTimeField struct {
	Field // Embedding the Field struct for reusability
}

func datetimeisoformat(d time.Time) string {
	return d.UTC().Format("2006-01-02T15:04:05.000000Z")
}

func NewDateTimeField(name string) *DateTimeField {
	return &DateTimeField{
		Field: Field{FieldExpr: FieldExpr{Name: name}},
	}
}

func (d *DateTimeField) Hour() *DerivedField {
	return NewDerivedField(&d.Field, "hour")
}

func (d *DateTimeField) Minute() *DerivedField {
	return NewDerivedField(&d.Field, "minute")
}

func (d *DateTimeField) Second() *DerivedField {
	return NewDerivedField(&d.Field, "second")
}

func (df *DateTimeField) Before(d time.Time) *BooleanFieldExpr {
	literalField := NewLiteralField(datetimeisoformat(d))
	return NewBooleanFieldExpr(df, literalField, Lt)
}

func (df *DateTimeField) After(d time.Time) *BooleanFieldExpr {
	literalField := NewLiteralField(datetimeisoformat(d))
	return NewBooleanFieldExpr(df, literalField, Gt)
}

func (df *DateTimeField) LessThan(d time.Time) *BooleanFieldExpr {
	return df.Before(d)
}

func (df *DateTimeField) GreaterThan(d time.Time) *BooleanFieldExpr {
	return df.After(d)
}

func (df *DateTimeField) LessThanOrEqual(d time.Time) *BooleanFieldExpr {
	literalField := NewLiteralField(datetimeisoformat(d))
	return NewBooleanFieldExpr(df, literalField, Lte)
}

func (df *DateTimeField) GreaterThanOrEqual(d time.Time) *BooleanFieldExpr {
	literalField := NewLiteralField(datetimeisoformat(d))
	return NewBooleanFieldExpr(df, literalField, Gte)
}

func (df *DateTimeField) Equal(d time.Time) *BooleanFieldExpr {
	literalField := NewLiteralField(datetimeisoformat(d))
	return NewBooleanFieldExpr(df, literalField, Eq)
}

func (df *DateTimeField) NotEquals(d time.Time) *BooleanFieldExpr {
	literalField := NewLiteralField(datetimeisoformat(d))
	return NewBooleanFieldExpr(df, literalField, Neq)
}

func AddDateTimeDuration(t time.Time, delta int, granularity DateTimeGranularity) time.Time {
	switch granularity {
	case DateTimeDaysGranularity:
		return t.AddDate(0, 0, delta)
	case DateTimeWeeksGranularity:
		return t.AddDate(0, 0, delta*7) // 7 days in a week
	case DateTimeMonthsGranularity:
		return t.AddDate(0, delta, 0)
	case DateTimeYearsGranularity:
		return t.AddDate(delta, 0, 0)
	case DateTimeHoursGranularity:
		return t.Add(time.Duration(delta) * time.Hour)
	case DateTimeMinutesGranularity:
		return t.Add(time.Duration(delta) * time.Minute)
	case DateTimeSecondsGranularity:
		return t.Add(time.Duration(delta) * time.Second)
	case DateTimeMillisecondsGranularity:
		return t.Add(time.Duration(delta) * time.Millisecond)
	default:
		return t
	}
}

func (dtf *DateTimeField) InPast(olderThan, newerThan int, granularity DateTimeGranularity) *BooleanFieldExpr {
	if olderThan > newerThan {
		log.Printf("in_past specified with older_than(%d) > newer_than(%d), swapping arguments.", olderThan, newerThan)
		olderThan, newerThan = newerThan, olderThan
	}

	now := time.Now().UTC()
	upper := AddDateTimeDuration(now, -olderThan, granularity)
	lower := AddDateTimeDuration(now, -newerThan, granularity)

	return dtf.GreaterThanOrEqual(lower).And(dtf.LessThanOrEqual(upper))
}

type ArrayField struct {
	Field
}

func NewArrayField(name string) *ArrayField {
	return &ArrayField{
		Field: Field{FieldExpr: FieldExpr{Name: name}},
	}
}

// hasAny returns a BooleanFieldExpr that checks if any of the values in vals are present in the array field
func (af *ArrayField) HasAny(vals []interface{}) *BooleanFieldExpr {
	return NewBooleanFieldExpr(af, NewLiteralField(vals), "hasAny")
}

// hasAll returns a BooleanFieldExpr that checks if all of the values in vals are present in the array field
func (af *ArrayField) HasAll(vals []interface{}) *BooleanFieldExpr {
	return NewBooleanFieldExpr(af, NewLiteralField(vals), "hasAll")
}

// UnsupportedOperationsError is used to handle operations that are not supported on ArrayField
type UnsupportedOperationsError struct {
	Operation string
}

func (e *UnsupportedOperationsError) Error() string {
	return fmt.Sprintf("Cannot call %s on array field", e.Operation)
}

// Attempt to handle unsupported operations with runtime checks
func (af *ArrayField) Max() error {
	return &UnsupportedOperationsError{Operation: "max"}
}

func (af *ArrayField) Min() error {
	return &UnsupportedOperationsError{Operation: "min"}
}

func (af *ArrayField) Sum() error {
	return &UnsupportedOperationsError{Operation: "sum"}
}

func (af *ArrayField) Count() error {
	return &UnsupportedOperationsError{Operation: "count"}
}

func (af *ArrayField) CountDistinct() error {
	return &UnsupportedOperationsError{Operation: "count_distinct"}
}

func (af *ArrayField) Avg() error {
	return &UnsupportedOperationsError{Operation: "avg"}
}

func (af *ArrayField) AvgDistinct() error {
	return &UnsupportedOperationsError{Operation: "avg_distinct"}
}

func (af *ArrayField) Equals() error {
	return &UnsupportedOperationsError{Operation: "=="}
}

func (af *ArrayField) NotEquals() error {
	return &UnsupportedOperationsError{Operation: "!="}
}

func (af *ArrayField) GreaterThan() error {
	return &UnsupportedOperationsError{Operation: ">"}
}

func (af *ArrayField) GreaterThanOrEquals() error {
	return &UnsupportedOperationsError{Operation: ">="}
}

func (af *ArrayField) LessThan() error {
	return &UnsupportedOperationsError{Operation: "<"}
}

func (af *ArrayField) LessThanOrEquals() error {
	return &UnsupportedOperationsError{Operation: "<="}
}

func (af *ArrayField) IsIn() error {
	return &UnsupportedOperationsError{Operation: "is_in"}
}

func (af *ArrayField) Between() error {
	return &UnsupportedOperationsError{Operation: "between"}
}
