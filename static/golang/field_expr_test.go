package main

import (
	"testing"
)

func TestFieldExpr(t *testing.T) {
	field := NewFieldExpr("field1")

	if field.Name != "field1" {
		t.Errorf("Expected field1 Name, got %v", field.Name)
	}
	if field.Alias != nil {
		t.Errorf("Expected field1 Alias, to be empty")
	}

	alias := "alias1"

	fieldWithAlias := NewFieldExpr("field1", alias)

	if fieldWithAlias.Name != "field1" {
		t.Errorf("Expected field1 Name, got %v", fieldWithAlias.Name)
	}
	if *fieldWithAlias.Alias != alias {
		t.Errorf("Expected field1 Alias, got %s", *fieldWithAlias.Alias)
	}
}

func TestFieldExprToString(t *testing.T) {
	field := NewFieldExpr("field1")

	if field.ToString() != "field1" {
		t.Errorf("Expected field1, got %v", field.ToString())
	}
}

func TestBooleanFieldExpr(t *testing.T) {
	field1 := NewFieldExpr("field1")
	field2 := NewFieldExpr("field2")
	boolExpr := NewBooleanFieldExpr(*field1, *field2, Eq)

	if boolExpr.Name != "field1" {
		t.Errorf("Expected name 'field1', got %v", field1.Name)
	}
	if boolExpr.Other.(FieldExpr).Name != "field2" {
		t.Errorf("Expected name 'field2', got %v", field2.Name)
	}
	if boolExpr.Field != *field1 {
		t.Errorf("Expected field1, got %v", boolExpr.Field)
	}
	if boolExpr.Op != "eq" {
		t.Errorf("Expected eq, got %v", boolExpr.Op)
	}
	if boolExpr.Other != *field2 {
		t.Errorf("Expected field2, got %v", boolExpr.Other)
	}
}

func TestBooleanFieldExprAnd(t *testing.T) {
	field1 := NewFieldExpr("field1")
	field2 := NewFieldExpr("field2")
	boolExpr1 := NewBooleanFieldExpr(*field1, *field2, Eq)
	boolExpr2 := NewBooleanFieldExpr(*field1, *field2, Neq)

	andExpr := boolExpr1.And(boolExpr2)

	if andExpr.Field != boolExpr1 {
		t.Errorf("Expected boolExpr1, got %v", andExpr.Field)
	}
	if andExpr.Op != "and" {
		t.Errorf("Expected and, got %v", andExpr.Op)
	}
	if andExpr.Other != boolExpr2 {
		t.Errorf("Expected boolExpr2, got %v", andExpr.Other)
	}
}

func TestBooleanFieldExprOr(t *testing.T) {
	field1 := NewFieldExpr("field1")
	field2 := NewFieldExpr("field2")
	boolExpr1 := NewBooleanFieldExpr(*field1, *field2, Eq)
	boolExpr2 := NewBooleanFieldExpr(*field1, *field2, Neq)
	orExpr := boolExpr1.Or(boolExpr2)

	if orExpr.Field != boolExpr1 {
		t.Errorf("Expected boolExpr1, got %v", orExpr.Field)
	}
	if orExpr.Op != "or" {
		t.Errorf("Expected or, got %v", orExpr.Op)
	}
	if orExpr.Other != boolExpr2 {
		t.Errorf("Expected boolExpr2, got %v", orExpr.Other)
	}
}

func TestUnaryBooleanFieldExpr(t *testing.T) {
	field := NewFieldExpr("field")
	unaryExpr := NewUnaryBooleanFieldExpr(*field, IsNull)

	if unaryExpr.Name != "(isNull(field))" {
		t.Errorf("Expected name '(isNull(field))', got %v", unaryExpr.Name)
	}
	if unaryExpr.Field != *field {
		t.Errorf("Expected field 'field', got %v", unaryExpr.Field)
	}
	if unaryExpr.Op != IsNull {
		t.Errorf("Expected operator 'isNull', got %v", unaryExpr.Op)
	}
}

func TestUnaryBooleanExprAnd(t *testing.T) {
	field := NewFieldExpr("field")
	unaryExpr := NewUnaryBooleanFieldExpr(*field, IsNotNull)
	andExpr := unaryExpr.And(*field)

	if andExpr.Field != unaryExpr.Field {
		t.Errorf("Expected field of unaryExpr, got %v", andExpr.Field)
	}
	if andExpr.Op != And {
		t.Errorf("Expected operator 'and', got %v", andExpr.Op)
	}
	if andExpr.Other != *field {
		t.Errorf("Expected other field 'field', got %v", andExpr.Other)
	}
}

func TestUnaryBooleanExprOr(t *testing.T) {
	field := NewFieldExpr("field")
	unaryExpr := NewUnaryBooleanFieldExpr(*field, IsNotNull)
	orExpr := unaryExpr.Or(*field)

	if orExpr.Field != unaryExpr.Field {
		t.Errorf("Expected field of unaryExpr, got %v", orExpr.Field)
	}
	if orExpr.Op != Or {
		t.Errorf("Expected operator 'or', got %v", orExpr.Op)
	}
	if orExpr.Other != *field {
		t.Errorf("Expected other field 'field', got %v", orExpr.Other)
	}
}

func TestAggregateFieldExpr(t *testing.T) {
	field := NewFieldExpr("field")
	aggExpr := NewAggregateFieldExpr(*field, Min)

	if aggExpr.Name != "(min(field))" {
		t.Errorf("Expected name '(min(field))', got %v", aggExpr.Name)
	}
	if aggExpr.Field != *field {
		t.Errorf("Expected field 'field', got %v", aggExpr.Field)
	}
	if aggExpr.Op != Min {
		t.Errorf("Expected operator 'min', got %v", aggExpr.Op)
	}
}

func TestAggregateFieldExprWithAlias(t *testing.T) {
	field := NewFieldExpr("field")
	aggExpr := NewAggregateFieldExpr(*field, Count)
	alias := "alias"
	aggExprWithAlias := aggExpr.WithAlias(alias)

	if aggExprWithAlias.Name != "(count(field))" {
		t.Errorf("Expected name '(count(field))', got %v", aggExprWithAlias.Name)
	}
	if aggExprWithAlias.Field != *field {
		t.Errorf("Expected field 'field', got %v", aggExprWithAlias.Field)
	}
	if aggExprWithAlias.Op != Count {
		t.Errorf("Expected operator 'count', got %v", aggExprWithAlias.Op)
	}
	if aggExprWithAlias.Alias == nil || *aggExprWithAlias.Alias != alias {
		t.Errorf("Expected alias 'alias', got %v", aggExprWithAlias.Alias)
	}
}
