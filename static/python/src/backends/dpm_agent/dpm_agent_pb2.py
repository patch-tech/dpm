# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: dpm_agent.proto
"""Generated protocol buffer code."""
from google.protobuf.internal import builder as _builder
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x0f\x64pm_agent.proto\x12\tdpm_agent\"n\n\x19SnowflakeConnectionParams\x12\x0c\n\x04user\x18\x01 \x01(\t\x12\x10\n\x08password\x18\x02 \x01(\t\x12\x0f\n\x07\x61\x63\x63ount\x18\x03 \x01(\t\x12\x10\n\x08\x64\x61tabase\x18\x04 \x01(\t\x12\x0e\n\x06schema\x18\x05 \x01(\t\"r\n\x11\x43onnectionRequest\x12I\n\x19snowflakeConnectionParams\x18\x01 \x01(\x0b\x32$.dpm_agent.SnowflakeConnectionParamsH\x00\x42\x12\n\x10\x63onnectionParams\"*\n\x12\x43onnectionResponse\x12\x14\n\x0c\x63onnectionId\x18\x01 \x01(\t\"\xde\x10\n\x05Query\x12\x14\n\x0c\x63onnectionId\x18\x01 \x01(\t\x12\x12\n\nselectFrom\x18\x02 \x01(\t\x12\x31\n\x06select\x18\x03 \x03(\x0b\x32!.dpm_agent.Query.SelectExpression\x12\x37\n\x06\x66ilter\x18\x04 \x01(\x0b\x32\".dpm_agent.Query.BooleanExpressionH\x00\x88\x01\x01\x12\x33\n\x07groupBy\x18\x05 \x03(\x0b\x32\".dpm_agent.Query.GroupByExpression\x12\x33\n\x07orderBy\x18\x06 \x03(\x0b\x32\".dpm_agent.Query.OrderByExpression\x12\x12\n\x05limit\x18\x07 \x01(\x04H\x01\x88\x01\x01\x12\x13\n\x06\x64ryRun\x18\x08 \x01(\x08H\x02\x88\x01\x01\x1a_\n\x10SelectExpression\x12-\n\x08\x61rgument\x18\x01 \x01(\x0b\x32\x1b.dpm_agent.Query.Expression\x12\x12\n\x05\x61lias\x18\x02 \x01(\tH\x00\x88\x01\x01\x42\x08\n\x06_alias\x1a\xa1\x02\n\nExpression\x12\x30\n\x05\x66ield\x18\x01 \x01(\x0b\x32\x1f.dpm_agent.Query.FieldReferenceH\x00\x12+\n\x07literal\x18\x02 \x01(\x0b\x32\x18.dpm_agent.Query.LiteralH\x00\x12\x35\n\x07\x64\x65rived\x18\x03 \x01(\x0b\x32\".dpm_agent.Query.DerivedExpressionH\x00\x12\x39\n\taggregate\x18\x04 \x01(\x0b\x32$.dpm_agent.Query.AggregateExpressionH\x00\x12\x37\n\tcondition\x18\x05 \x01(\x0b\x32\".dpm_agent.Query.BooleanExpressionH\x00\x42\t\n\x07\x65x_type\x1a#\n\x0e\x46ieldReference\x12\x11\n\tfieldName\x18\x01 \x01(\t\x1a\x90\x02\n\x07Literal\x12\x10\n\x06string\x18\x01 \x01(\tH\x00\x12\x11\n\x07\x62oolean\x18\x02 \x01(\x08H\x00\x12\r\n\x03i32\x18\x03 \x01(\rH\x00\x12\x0e\n\x04ui64\x18\x04 \x01(\x04H\x00\x12\x0e\n\x04ui32\x18\x05 \x01(\x05H\x00\x12\r\n\x03i64\x18\x06 \x01(\x03H\x00\x12\r\n\x03\x66\x33\x32\x18\x07 \x01(\x02H\x00\x12\r\n\x03\x66\x36\x34\x18\x08 \x01(\x01H\x00\x12\x13\n\ttimestamp\x18\t \x01(\x03H\x00\x12-\n\x04list\x18\n \x01(\x0b\x32\x1d.dpm_agent.Query.Literal.ListH\x00\x1a\x30\n\x04List\x12(\n\x06values\x18\x01 \x03(\x0b\x32\x18.dpm_agent.Query.LiteralB\x0e\n\x0cliteral_type\x1a\x80\x02\n\x11\x44\x65rivedExpression\x12\x41\n\x02op\x18\x01 \x01(\x0e\x32\x35.dpm_agent.Query.DerivedExpression.ProjectionOperator\x12-\n\x08\x61rgument\x18\x02 \x01(\x0b\x32\x1b.dpm_agent.Query.Expression\"y\n\x12ProjectionOperator\x12\x08\n\x04YEAR\x10\x00\x12\t\n\x05MONTH\x10\x01\x12\x07\n\x03\x44\x41Y\x10\x02\x12\x08\n\x04HOUR\x10\x03\x12\n\n\x06MINUTE\x10\x04\x12\n\n\x06SECOND\x10\x05\x12\x0f\n\x0bMILLISECOND\x10\x06\x12\x08\n\x04\x44\x41TE\x10\x07\x12\x08\n\x04TIME\x10\x08\x1a\xed\x01\n\x13\x41ggregateExpression\x12\x42\n\x02op\x18\x01 \x01(\x0e\x32\x36.dpm_agent.Query.AggregateExpression.AggregateOperator\x12-\n\x08\x61rgument\x18\x02 \x01(\x0b\x32\x1b.dpm_agent.Query.Expression\"c\n\x11\x41ggregateOperator\x12\x07\n\x03MIN\x10\x00\x12\x07\n\x03MAX\x10\x01\x12\x08\n\x04MEAN\x10\x02\x12\n\n\x06MEDIAN\x10\x03\x12\t\n\x05\x43OUNT\x10\x04\x12\x12\n\x0e\x43OUNT_DISTINCT\x10\x05\x12\x07\n\x03SUM\x10\x06\x1a\x98\x02\n\x11\x42ooleanExpression\x12>\n\x02op\x18\x01 \x01(\x0e\x32\x32.dpm_agent.Query.BooleanExpression.BooleanOperator\x12.\n\targuments\x18\x02 \x03(\x0b\x32\x1b.dpm_agent.Query.Expression\"\x92\x01\n\x0f\x42ooleanOperator\x12\x07\n\x03\x41ND\x10\x00\x12\x06\n\x02OR\x10\x01\x12\x06\n\x02\x45Q\x10\x02\x12\x07\n\x03NEQ\x10\x03\x12\x06\n\x02LT\x10\x04\x12\x07\n\x03LTE\x10\x05\x12\x06\n\x02GT\x10\x06\x12\x07\n\x03GTE\x10\x07\x12\x08\n\x04LIKE\x10\x08\x12\x0b\n\x07\x42\x45TWEEN\x10\t\x12\x06\n\x02IN\x10\n\x12\x0b\n\x07IS_NULL\x10\x0b\x12\x0f\n\x0bIS_NOT_NULL\x10\x0c\x1a\x87\x01\n\x11GroupByExpression\x12\x30\n\x05\x66ield\x18\x01 \x01(\x0b\x32\x1f.dpm_agent.Query.FieldReferenceH\x00\x12\x35\n\x07\x64\x65rived\x18\x02 \x01(\x0b\x32\".dpm_agent.Query.DerivedExpressionH\x00\x42\t\n\x07\x65x_type\x1a\xb6\x01\n\x11OrderByExpression\x12-\n\x08\x61rgument\x18\x01 \x01(\x0b\x32\x1b.dpm_agent.Query.Expression\x12\x44\n\tdirection\x18\x02 \x01(\x0e\x32,.dpm_agent.Query.OrderByExpression.DirectionH\x00\x88\x01\x01\"\x1e\n\tDirection\x12\x07\n\x03\x41SC\x10\x00\x12\x08\n\x04\x44\x45SC\x10\x01\x42\x0c\n\n_directionB\t\n\x07_filterB\x08\n\x06_limitB\t\n\x07_dryRun\"4\n\x0bQueryResult\x12\x13\n\x0bqueryString\x18\x01 \x01(\t\x12\x10\n\x08jsonData\x18\x02 \x01(\t\")\n\x11\x44isconnectRequest\x12\x14\n\x0c\x63onnectionId\x18\x01 \x01(\t\"\x14\n\x12\x44isconnectResponse2\xf0\x01\n\x08\x44pmAgent\x12Q\n\x10\x43reateConnection\x12\x1c.dpm_agent.ConnectionRequest\x1a\x1d.dpm_agent.ConnectionResponse\"\x00\x12:\n\x0c\x45xecuteQuery\x12\x10.dpm_agent.Query\x1a\x16.dpm_agent.QueryResult\"\x00\x12U\n\x14\x44isconnectConnection\x12\x1c.dpm_agent.DisconnectRequest\x1a\x1d.dpm_agent.DisconnectResponse\"\x00\x62\x06proto3')

_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, globals())
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'dpm_agent_pb2', globals())
if _descriptor._USE_C_DESCRIPTORS == False:

  DESCRIPTOR._options = None
  _SNOWFLAKECONNECTIONPARAMS._serialized_start=30
  _SNOWFLAKECONNECTIONPARAMS._serialized_end=140
  _CONNECTIONREQUEST._serialized_start=142
  _CONNECTIONREQUEST._serialized_end=256
  _CONNECTIONRESPONSE._serialized_start=258
  _CONNECTIONRESPONSE._serialized_end=300
  _QUERY._serialized_start=303
  _QUERY._serialized_end=2445
  _QUERY_SELECTEXPRESSION._serialized_start=609
  _QUERY_SELECTEXPRESSION._serialized_end=704
  _QUERY_EXPRESSION._serialized_start=707
  _QUERY_EXPRESSION._serialized_end=996
  _QUERY_FIELDREFERENCE._serialized_start=998
  _QUERY_FIELDREFERENCE._serialized_end=1033
  _QUERY_LITERAL._serialized_start=1036
  _QUERY_LITERAL._serialized_end=1308
  _QUERY_LITERAL_LIST._serialized_start=1244
  _QUERY_LITERAL_LIST._serialized_end=1292
  _QUERY_DERIVEDEXPRESSION._serialized_start=1311
  _QUERY_DERIVEDEXPRESSION._serialized_end=1567
  _QUERY_DERIVEDEXPRESSION_PROJECTIONOPERATOR._serialized_start=1446
  _QUERY_DERIVEDEXPRESSION_PROJECTIONOPERATOR._serialized_end=1567
  _QUERY_AGGREGATEEXPRESSION._serialized_start=1570
  _QUERY_AGGREGATEEXPRESSION._serialized_end=1807
  _QUERY_AGGREGATEEXPRESSION_AGGREGATEOPERATOR._serialized_start=1708
  _QUERY_AGGREGATEEXPRESSION_AGGREGATEOPERATOR._serialized_end=1807
  _QUERY_BOOLEANEXPRESSION._serialized_start=1810
  _QUERY_BOOLEANEXPRESSION._serialized_end=2090
  _QUERY_BOOLEANEXPRESSION_BOOLEANOPERATOR._serialized_start=1944
  _QUERY_BOOLEANEXPRESSION_BOOLEANOPERATOR._serialized_end=2090
  _QUERY_GROUPBYEXPRESSION._serialized_start=2093
  _QUERY_GROUPBYEXPRESSION._serialized_end=2228
  _QUERY_ORDERBYEXPRESSION._serialized_start=2231
  _QUERY_ORDERBYEXPRESSION._serialized_end=2413
  _QUERY_ORDERBYEXPRESSION_DIRECTION._serialized_start=2369
  _QUERY_ORDERBYEXPRESSION_DIRECTION._serialized_end=2399
  _QUERYRESULT._serialized_start=2447
  _QUERYRESULT._serialized_end=2499
  _DISCONNECTREQUEST._serialized_start=2501
  _DISCONNECTREQUEST._serialized_end=2542
  _DISCONNECTRESPONSE._serialized_start=2544
  _DISCONNECTRESPONSE._serialized_end=2564
  _DPMAGENT._serialized_start=2567
  _DPMAGENT._serialized_end=2807
# @@protoc_insertion_point(module_scope)
