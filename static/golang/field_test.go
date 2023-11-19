package main

import (
	"fmt"
	"reflect"
	"testing"
	"time"
)

// TestFieldWithAliasEnsuresImmutability checks that using WithAlias
// returns a copy of the Field and does not mutate the original Field.
func TestFieldWithAliasEnsuresImmutability(t *testing.T) {
	field := NewField("a_number")
	aliased := field.WithAlias("floaty")

	if &field.FieldExpr == aliased {
		t.Errorf("Expected field and aliased to be different instances, but they are the same")
	}

	if field.Name != "a_number" {
		t.Errorf("Expected field name to be 'a_number', got '%s'", field.Name)
	}

	if aliased.Name != "a_number" {
		t.Errorf("Expected aliased field name to be 'a_number', got '%s'", aliased.Name)
	}

	if field.Alias != nil {
		t.Errorf("Expected field alias to be nil, got '%v'", &field.Alias)
	}

	if aliased.Alias == nil || *aliased.Alias != "floaty" {
		t.Errorf("Expected aliased field alias to be 'floaty', got '%v'", aliased.Alias)
	}
}

func TestFieldHasTheExpectedOperatorAndOperands(t *testing.T) {
	field := NewField("invoice_id")
	if field.Operator() != "ident" {
		t.Errorf("Expected operator to be 'ident', got %s", field.Operator())
	}

	operands := field.Operands()
	if len(operands) != 1 || operands[0].(StringExpr).Value != "invoice_id" {
		t.Errorf("Expected operands to be [%v], got [%v]", field, operands)
	}
}

func TestFieldReturnsTheCorrectAggregateExpression(t *testing.T) {
	price := NewField("price")

	testAggregateExpression := func(aggExpr *AggregateFieldExpr, expectedOperator string) {
		operands := aggExpr.Operands()
		if len(operands) != 1 {
			t.Fatalf("Expected one operand, got %d", len(operands))
		}

		operand, ok := operands[0].(*FieldExpr)
		if !ok {
			t.Fatalf("Operand is not of type *Field, got %T", operands[0])
		}

		if operand.Name != price.Name {
			t.Errorf("Expected operand to be %v, got %v", price.Name, operand.Name)
		}

		if *operand != price.FieldExpr {
			t.Errorf("Expected operand to be %v, got %v", price, operand)
		}

		if string(aggExpr.Operator()) != expectedOperator {
			t.Errorf("Expected operator to be '%s', got '%s'", expectedOperator, aggExpr.Operator())
		}
	}

	maxPrice := price.AvgDistinct()
	if reflect.TypeOf(maxPrice) != reflect.TypeOf(&AggregateFieldExpr{}) {
		t.Errorf("Expected maxPrice to be of type *AggregateFieldExpr, got %T", maxPrice)
	}
	testAggregateExpression(maxPrice, "avgDistinct")

	totalPrice := price.Sum()
	if reflect.TypeOf(totalPrice) != reflect.TypeOf(&AggregateFieldExpr{}) {
		t.Errorf("Expected totalPrice to be of type *AggregateFieldExpr, got %T", totalPrice)
	}
	testAggregateExpression(totalPrice, "sum")
}

func TestDerivedFieldAsReturnsCopyAndDoesNotMutate(t *testing.T) {
	startedOn := NewField("startedOn")
	startedOnYear := NewDerivedField(startedOn, "year")

	aliased := startedOnYear.WithAlias("startedOnYear")

	if startedOnYear == aliased {
		t.Errorf("Expected startedOnYear and aliased to be different instances")
	}

	if startedOnYear.Name != "(year(startedOn))" {
		t.Errorf("Expected name of startedOnYear to be '(year(startedOn))', got %s", startedOnYear.Name)
	}

	if aliased.Name != "(year(startedOn))" {
		t.Errorf("Expected name of aliased to be '(year(startedOn))', got %s", aliased.Name)
	}

	if startedOnYear.Alias != nil {
		t.Errorf("Expected alias of startedOnYear to be nil")
	}

	if *aliased.Alias != "startedOnYear" {
		t.Errorf("Expected alias of aliased to be 'startedOnYear', got %s", *aliased.Alias)
	}
}

func TestDateFieldBooleanOperationReturnsExpectedBooleanExpression(t *testing.T) {
	d := NewDateField("started_on_date")

	dateToCompare := time.Date(2023, 11, 1, 0, 0, 0, 0, time.UTC)
	boolExpr := d.After(dateToCompare)

	dateField, ok := boolExpr.Field.(*DateField)
	if !ok {
		t.Fatalf("Expected boolExpr.Field to be *DateField, got %T", boolExpr.Field)
	}
	expected_name := dateField.Name

	if expected_name != "started_on_date" {
		t.Errorf("Expected boolExpr.Field name to be 'started_on_date', got %v", expected_name)
	}

	if boolExpr.Op != Gt {
		t.Errorf("Expected boolExpr.Op to be Gt, got %v", boolExpr.Op)
	}

	literalValue, ok := boolExpr.Other.(*LiteralField)
	if !ok {
		t.Fatalf("Expected boolExpr.Other.Value to be a string, got %T", literalValue)
	}

	expectedDateString := "2023-11-01"
	if literalValue.Value != expectedDateString {
		t.Errorf("Expected boolExpr.Other.Value to be '%s', got '%v'", expectedDateString, literalValue.Value)
	}
}

func TestTimeFieldBooleanOperation(t *testing.T) {
	tField := NewTimeField("started_at_time")

	// Create a time object to compare with
	comparisonTime := time.Date(0, 0, 0, 12, 8, 7, 0, time.UTC)

	// Perform the boolean operation
	boolExpr := tField.LessThanOrEqual(comparisonTime)

	// Check if boolExpr.field is of type TimeField
	timeField, ok := boolExpr.Field.(*TimeField)
	if !ok {
		t.Errorf("boolExpr.Field is not of type TimeField, got %T", boolExpr.Field)
	}

	// Check the name of the field
	if timeField.Name != "started_at_time" {
		t.Errorf("Expected field name 'started_at_time', got '%s'", boolExpr.Field.(FieldExpr).Name)
	}

	// Check the operator
	if boolExpr.Op != Lte {
		t.Errorf("Expected operator 'lte', got '%s'", boolExpr.Op)
	}

	// Check if boolExpr.other is of type LiteralField
	if otherLiteral, ok := boolExpr.Other.(*LiteralField); ok {
		// Check the value of the literal field
		if otherLiteral.Value != "12:08:07" {
			t.Errorf("Expected literal field value '12:08:07', got '%v'", otherLiteral.Value)
		}
	} else {
		t.Errorf("boolExpr.Other is not of type *LiteralField, got %T", boolExpr.Other)
	}
}

func TestDateTimeFieldBooleanOperation(t *testing.T) {
	// Create a new DateTimeField
	dt := NewDateTimeField("started_at_time")

	// Parse the ISO format date string
	dateToCompare, err := time.Parse(time.RFC3339, "2023-11-01T12:08:07-07:53")
	if err != nil {
		t.Fatal(err)
	}

	// Perform the LessThanOrEqual operation
	boolExpr := dt.LessThanOrEqual(dateToCompare)

	// Check if boolExpr.Field is of type *DateTimeField and has the expected name
	dtField, ok := boolExpr.Field.(*DateTimeField)
	if !ok || dtField.Name != "started_at_time" {
		t.Errorf("Expected DateTimeField with name 'started_at_time', got: %T with name '%s'", boolExpr.Field, "foo")
	}

	// Check the operator
	if boolExpr.Op != Lte {
		t.Errorf("Expected operator to be 'lte', got: '%s'", boolExpr.Op)
	}

	// Check if boolExpr.Other is of type *LiteralField and has the expected value
	if litField, ok := boolExpr.Other.(*LiteralField); !ok || fmt.Sprint(litField.Value) != "2023-11-01T20:01:07.000000Z" {
		t.Errorf("Expected LiteralField with value '2023-11-01T20:01:07.000000Z', got: %T with value '%v'", boolExpr.Other, litField.Value)
	}
}

func TestDateTimeFieldBooleanOperationReturnsExpectedBooleanExpression(t *testing.T) {
	dt := NewDateTimeField("started_at_time")

	// Parsing the provided datetime string
	dateToCompare, err := time.Parse(time.RFC3339, "2023-11-01T12:08:07-07:53")
	if err != nil {
		t.Fatalf("Failed to parse datetime: %v", err)
	}

	boolExpr := dt.LessThanOrEqual(dateToCompare)

	// Check if boolExpr.field is of type *DateTimeField
	if _, ok := boolExpr.Field.(*DateTimeField); !ok {
		t.Errorf("boolExpr.Field is not of type *DateTimeField")
	}

	// Check field name
	if boolExpr.Field.(*DateTimeField).Name != "started_at_time" {
		t.Errorf("Expected boolExpr.Field name to be 'started_at_time', got %v", boolExpr.Field.(*DateTimeField).Name)
	}

	// Check operation type
	if boolExpr.Op != Lte {
		t.Errorf("Expected boolExpr.Op to be Lte, got %v", boolExpr.Op)
	}

	// Check if boolExpr.other is of type *LiteralField
	literalValue, ok := boolExpr.Other.(*LiteralField)
	if !ok {
		t.Fatalf("boolExpr.Other is not of type *LiteralField")
	}

	// Check literal value
	expectedDateString := "2023-11-01T20:01:07.000000Z" // UTC representation of the provided datetime
	if literalValue.Value != expectedDateString {
		t.Errorf("Expected boolExpr.Other.Value to be '%s', got '%v'", expectedDateString, literalValue.Value)
	}
}
