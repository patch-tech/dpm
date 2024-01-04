package backends

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

	if field == aliased {
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

		operand, ok := operands[0].(*Field)
		if !ok {
			t.Fatalf("Operand is not of type *Field, got %T", operands[0])
		}

		if operand.Name != price.Name {
			t.Errorf("Expected operand to be %v, got %v", price.Name, operand.Name)
		}

		if operand != price {
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

func TestAddDurationReturnsExpectedResults(t *testing.T) {

	testDuration := func(name string, addFunc interface{}, input time.Time, delta int, unit string, expected time.Time) {
		t.Run(name, func(t *testing.T) {
			var result time.Time
			switch f := addFunc.(type) {
			case func(time.Time, int, DateGranularity) time.Time:
				result = f(input, delta, DateGranularity(unit))
			case func(time.Time, int, TimeGranularity) time.Time:
				result = f(input, delta, TimeGranularity(unit))
			case func(time.Time, int, DateTimeGranularity) time.Time:
				result = f(input, delta, DateTimeGranularity(unit))
			default:
				t.Fatalf("Invalid function type")
			}

			if !result.Equal(expected) {
				t.Errorf("Expected %v, got %v", expected, result)
			}
		})
	}
	// Test cases for Date
	// ...
	testDuration("Subtract 1 year from 2023-10-12", AddDateDuration, time.Date(2023, 10, 12, 0, 0, 0, 0, time.UTC), -1, "years", time.Date(2022, 10, 12, 0, 0, 0, 0, time.UTC))
	testDuration("Add 13 days to 2023-02-15", AddDateDuration, time.Date(2023, 2, 15, 0, 0, 0, 0, time.UTC), 13, "days", time.Date(2023, 2, 28, 0, 0, 0, 0, time.UTC))
	testDuration("Add 2 weeks to 2023-02-15", AddDateDuration, time.Date(2023, 2, 15, 0, 0, 0, 0, time.UTC), 2, "weeks", time.Date(2023, 3, 1, 0, 0, 0, 0, time.UTC))

	// Test cases for Time
	testDuration("Subtract 16 hours from 15:02:45 clamping to zero", AddTimeDuration, time.Date(0, 0, 0, 15, 2, 45, 0, time.UTC), -16, "hours", time.Date(0, 0, 0, 0, 0, 0, 0, time.UTC))
	testDuration("Subtract 1024 hours from 15:02:45 clamping to zero", AddTimeDuration, time.Date(0, 0, 0, 15, 2, 45, 0, time.UTC), -1024, "hours", time.Date(0, 0, 0, 0, 0, 0, 0, time.UTC))
	testDuration("Add 9 hours to 15:02:45 clamping to last time of day", AddTimeDuration, time.Date(0, 0, 0, 15, 2, 45, 0, time.UTC), 9, "hours", time.Date(0, 0, 0, 23, 59, 59, 999999000, time.UTC))
	testDuration("Add 9000 hours to 15:02:45 clamping to last time of day", AddTimeDuration, time.Date(0, 0, 0, 15, 2, 45, 0, time.UTC), 9000, "hours", time.Date(0, 0, 0, 23, 59, 59, 999999000, time.UTC))
	testDuration("Subtract 12 minutes from 15:02:45", AddTimeDuration, time.Date(0, 0, 0, 15, 2, 45, 0, time.UTC), -12, "minutes", time.Date(0, 0, 0, 14, 50, 45, 0, time.UTC))
	testDuration("Add 15000 milliseconds to 15:02:45", AddTimeDuration, time.Date(0, 0, 0, 15, 2, 45, 0, time.UTC), 15000, "milliseconds", time.Date(0, 0, 0, 15, 3, 0, 0, time.UTC))

	// Test cases for DateTime
	dt := time.Date(2023, 2, 15, 15, 2, 45, 0, time.UTC)
	testDuration("Add -1 year to 2023-02-15 15:02:45", AddDateTimeDuration, dt, -1, "years", time.Date(2022, 2, 15, 15, 2, 45, 0, time.UTC))
	testDuration("Add 13 days to 2023-02-15 15:02:45", AddDateTimeDuration, dt, 13, "days", time.Date(2023, 2, 28, 15, 2, 45, 0, time.UTC))
	testDuration("Add 2 weeks to 2023-02-15 15:02:45", AddDateTimeDuration, dt, 2, "weeks", time.Date(2023, 3, 1, 15, 2, 45, 0, time.UTC))
	testDuration("Add 15123 milliseconds to 2023-02-15 15:02:45", AddDateTimeDuration, dt, 15123, "milliseconds", time.Date(2023, 2, 15, 15, 3, 0, 123000000, time.UTC))
}

func TestDateFieldInPastReturnsExpectedBooleanExpression(t *testing.T) {
	d := NewDateField("started_on_date")
	boolExpr := d.InPast(1, 2, WeeksGranularity) // assuming WeeksGranularity is defined

	assertEqual(t, string(boolExpr.Operator()), "and")

	operands := boolExpr.Operands()
	if len(operands) != 2 {
		t.Fatalf("Expected 2 operands, got %d", len(operands))
	}

	operand1 := operands[0].(*BooleanFieldExpr)
	operand2 := operands[1].(*BooleanFieldExpr)

	assertEqual(t, string(operand1.Operator()), "gte")
	assertEqual(t, string(operand2.Operator()), "lte")

	lhs1 := operand1.Field.(*DateField)
	lhs2 := operand2.Field.(*DateField)

	assertEqual(t, lhs1.Name, "started_on_date")
	assertEqual(t, lhs2.Name, "started_on_date")

	lower := operand1.Other.(*LiteralField)
	upper := operand2.Other.(*LiteralField)

	lowerDate, errLower := time.Parse("2006-01-02", lower.Value.(string))
	upperDate, errUpper := time.Parse("2006-01-02", upper.Value.(string))

	if errLower != nil || errUpper != nil {
		t.Fatalf("DateField in_past produced invalid ranges '%v' or '%v'", errLower, errUpper)
	}

	if !lowerDate.Before(upperDate) && !lowerDate.Equal(upperDate) {
		t.Errorf("Expected lower date to be less than or equal to upper date")
	}

}

func assertEqual(t *testing.T, got, want interface{}) {
	if got != want {
		t.Errorf("Got %v, want %v", got, want)
	}
}

func TestTimeFieldInPastReturnsExpectedBooleanExpression(t *testing.T) {
	field := NewTimeField("started_at_time")
	boolExpr := field.InPast(1, 2, HoursGranularity)

	// Check top-level operator
	if string(boolExpr.Operator()) != string(And) {
		t.Errorf("Expected top-level operator to be And, got %v", boolExpr.Operator())
	}

	operands := boolExpr.Operands()
	if len(operands) != 2 {
		t.Fatalf("Expected 2 operands, got %d", len(operands))
	}

	// Check first operand
	operand1 := operands[0]
	if string(operand1.Operator()) != string(Gte) {
		t.Errorf("Expected operand 1 operator to be Gte, got %v", operand1.Operator())
	}

	// Check field of first operand
	lhs1 := operand1.Operands()[0]
	timeField1, ok := lhs1.(*TimeField)
	if !ok || timeField1.Name != "started_at_time" {
		t.Errorf("Expected LHS of operand 1 to be TimeField with name 'started_at_time', got %T with name %v", lhs1, timeField1.Name)
	}

	// Repeat checks for second operand
	operand2 := operands[1]
	if string(operand2.Operator()) != string(Lte) {
		t.Errorf("Expected operand 2 operator to be Lte, got %v", operand2.Operator())
	}

	lhs2 := operand2.Operands()[0]
	timeField2, ok := lhs2.(*TimeField)
	if !ok || timeField2.Name != "started_at_time" {
		t.Errorf("Expected LHS of operand 2 to be TimeField with name 'started_at_time', got %T with name %v", lhs2, timeField2.Name)
	}

	lowerField, ok := operand1.Operands()[1].(*LiteralField)
	if !ok {
		t.Fatalf("Expected RHS of operand 1 to be LiteralField, got %T", operand1.Operands()[1])
	}

	lowerTime, err := time.Parse(time.TimeOnly, lowerField.Value.(string))
	if err != nil {
		t.Fatalf("Failed to parse lower time: %v", err)
	}

	// Check RHS of second operand
	upperField, ok := operand2.Operands()[1].(*LiteralField)
	if !ok {
		t.Fatalf("Expected RHS of operand 2 to be LiteralField, got %T", operand2.Operands()[1])
	}

	upperTime, err := time.Parse(time.TimeOnly, upperField.Value.(string))
	if err != nil {
		t.Fatalf("Failed to parse upper time: %v", err)
	}

	// Compare the parsed times
	if !lowerTime.Before(upperTime) && !lowerTime.Equal(upperTime) {
		t.Errorf("Expected lower time to be before or equal to upper time, got lower: %v, upper: %v", lowerTime, upperTime)
	}
}

func TestDateTimeFieldInPastReturnsExpectedBooleanExpression(t *testing.T) {
	field := NewDateTimeField("started_at_time")
	boolExpr := field.InPast(1, 2, DateTimeYearsGranularity)

	// Check top-level operator
	if string(boolExpr.Operator()) != string(And) {
		t.Errorf("Expected top-level operator to be And, got %v", boolExpr.Operator())
	}

	operands := boolExpr.Operands()
	if len(operands) != 2 {
		t.Fatalf("Expected 2 operands, got %d", len(operands))
	}

	// Check first operand
	operand1 := operands[0]
	if string(operand1.Operator()) != string(Gte) {
		t.Errorf("Expected operand 1 operator to be Gte, got %v", operand1.Operator())
	}

	// Check field of first operand
	lhs1 := operand1.Operands()[0]
	timeField1, ok := lhs1.(*DateTimeField)
	if !ok || timeField1.Name != "started_at_time" {
		t.Errorf("Expected LHS of operand 1 to be DateTimeField with name 'started_at_time', got %T with name %v", lhs1, timeField1.Name)
	}

	// Repeat checks for second operand
	operand2 := operands[1]
	if string(operand2.Operator()) != string(Lte) {
		t.Errorf("Expected operand 2 operator to be Lte, got %v", operand2.Operator())
	}

	lhs2 := operand2.Operands()[0]
	timeField2, ok := lhs2.(*DateTimeField)
	if !ok || timeField2.Name != "started_at_time" {
		t.Errorf("Expected LHS of operand 2 to be DateTimeField with name 'started_at_time', got %T with name %v", lhs2, timeField2.Name)
	}

	lowerField, ok := operand1.Operands()[1].(*LiteralField)
	if !ok {
		t.Fatalf("Expected RHS of operand 1 to be LiteralField, got %T", operand1.Operands()[1])
	}

	lowerTime, err := time.Parse(time.RFC3339Nano, lowerField.Value.(string))
	if err != nil {
		t.Fatalf("Failed to parse lower time: %v", err)
	}

	// Check RHS of second operand
	upperField, ok := operand2.Operands()[1].(*LiteralField)
	if !ok {
		t.Fatalf("Expected RHS of operand 2 to be LiteralField, got %T", operand2.Operands()[1])
	}

	upperTime, err := time.Parse(time.RFC3339Nano, upperField.Value.(string))
	if err != nil {
		t.Fatalf("Failed to parse upper time: %v", err)
	}

	// Compare the parsed times
	if !lowerTime.Before(upperTime) && !lowerTime.Equal(upperTime) {
		t.Errorf("Expected lower time to be before or equal to upper time, got lower: %v, upper: %v", lowerTime, upperTime)
	}
}

func TestArrayFieldHasAnyReturnsExpectedBooleanExpression(t *testing.T) {
	tagNames := NewArrayField("tag_names")
	hasAnyTags := tagNames.HasAny([]interface{}{"foo", "bar"})

	if hasAnyTags.Operator() != "hasAny" {
		t.Errorf("Expected operator to be 'hasAny', got %v", hasAnyTags.Operator())
	}

	operands := hasAnyTags.Operands()
	if len(operands) != 2 {
		t.Fatalf("Expected 2 operands, got %d", len(operands))
	}

	operand1, ok := operands[0].(*ArrayField)
	if !ok {
		t.Fatalf("Expected first operand to be *ArrayField, got %T", operands[0])
	}
	if operand1.Name != "tag_names" {
		t.Errorf("Expected first operand name to be 'tag_names', got '%s'", operand1.Name)
	}

	operand2, ok := operands[1].(*LiteralField)
	if !ok {
		t.Fatalf("Expected second operand to be *LiteralField, got %T", operands[1])
	}
	if !reflect.DeepEqual(operand2.Value, []interface{}{"foo", "bar"}) {
		t.Errorf("Expected second operand value to be ['foo', 'bar'], got '%v'", operand2.Value)
	}
}

func TestArrayFieldHasAllReturnsExpectedBooleanExpression(t *testing.T) {
	tagNames := NewArrayField("tag_names")
	hasAllTags := tagNames.HasAll([]interface{}{"foo", "bar"})

	if hasAllTags.Operator() != "hasAll" {
		t.Errorf("Expected operator to be 'hasAll', got %v", hasAllTags.Operator())
	}

	operands := hasAllTags.Operands()
	if len(operands) != 2 {
		t.Fatalf("Expected 2 operands, got %d", len(operands))
	}

	operand1, ok := operands[0].(*ArrayField)
	if !ok {
		t.Fatalf("Expected first operand to be *ArrayField, got %T", operands[0])
	}
	if operand1.Name != "tag_names" {
		t.Errorf("Expected first operand name to be 'tag_names', got '%s'", operand1.Name)
	}

	operand2, ok := operands[1].(*LiteralField)
	if !ok {
		t.Fatalf("Expected second operand to be *LiteralField, got %T", operands[1])
	}
	if !reflect.DeepEqual(operand2.Value, []interface{}{"foo", "bar"}) {
		t.Errorf("Expected second operand value to be ['foo', 'bar'], got '%v'", operand2.Value)
	}
}
