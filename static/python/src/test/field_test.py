from ..field import DateField, DerivedField, Field, AggregateFieldExpr

def test_field_as_returns_copy_and_does_not_mutate_this():
    field = Field('a_number')
    aliased = field.with_alias('floaty')
    assert field is not aliased
    assert field.name == 'a_number'
    assert aliased.name == 'a_number'
    assert field.alias is None
    assert aliased.alias == 'floaty'

def test_field_has_the_expected_operator_and_operands():
    field = Field('invoice_id')
    assert field.operands() == ['invoice_id']
    assert field.operator() == 'ident'

def test_field_returns_the_correct_aggregate_expression():
    price = Field('price')
    max_price = price.avg_distinct()
    assert isinstance(max_price, AggregateFieldExpr)
    assert max_price.operands() == [price]
    assert max_price.operator() == 'avgDistinct'

def test_derived_field_as_returns_copy_of_derived_field_and_does_not_mutate_this():
    started_on = DateField('startedOn')
    started_on_year = started_on.year
    aliased = started_on_year.with_alias('startedOnYear')
    assert isinstance(started_on_year, DerivedField)
    assert started_on_year is not aliased
    assert started_on_year.name == '(year(startedOn))'
    assert aliased.name == '(year(startedOn))'
    assert started_on_year.alias is None
    assert aliased.alias == 'startedOnYear'