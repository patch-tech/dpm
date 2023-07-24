from ..backends.patch import snake_to_camel, field_as_graphql, selection_as_graphql, with_alias, format_default, format_unary
from ..field import DateField, StringField, Field, LiteralField, AggregateFieldExpr, UnaryBooleanFieldExpr

def test_snake_to_camel():
    assert snake_to_camel("hello_world") == "helloWorld"
    assert snake_to_camel("HELLO_WORLD") == "helloWorld"
    assert snake_to_camel("hello_WORLD") == "helloWorld"
    assert snake_to_camel("hello") == "hello"

def test_with_alias():
    assert with_alias("field") == "field"
    assert with_alias("field", "field_alias") == "field_alias: field"
    assert with_alias("field", "field") == "field: field"

def test_graphql_selection():
    price = Field("price")
    size = Field("size")
    assert selection_as_graphql([price, size]) == "price\nsize"

def test_field_as_graphql():
    price = LiteralField("price")
    size = StringField("size_of_beans")
    maxPrice = AggregateFieldExpr(price, "max")
    assert field_as_graphql(price) == "\"price\""
    assert field_as_graphql(size) == "sizeOfBeans"
    assert field_as_graphql(maxPrice).__contains__("\"price\"<bound method aggregatefieldexpr.operator")

def test_format():
     price = LiteralField("price")
     size = LiteralField("size_of_beans")

     expected_default_output = """
     {
    \"price\": {
      and: \"size_of_beans\"
    }
  }
    """
     assert format_default("and", price, size) == expected_default_output.strip()
