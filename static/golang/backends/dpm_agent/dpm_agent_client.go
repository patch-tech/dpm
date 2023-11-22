package dpm_agent

import (
	"context"
	"fmt"
	"log"
	"net/url"

	models "github.com/patch-tech/dpm/models"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
)

type authCreds struct {
	token string
}

type DpmAgentServiceClient struct {
	Client       DpmAgentClient
	DpmAuthToken string
}

// I defined this but Im not sure about this
type Timestamp int64

func makeLiteral(x interface{}) *Query_Literal {
	literal := &Query_Literal{}

	switch v := x.(type) {
	case string:
		literal.LiteralType = &Query_Literal_String_{String_: v}
	case bool:
		literal.LiteralType = &Query_Literal_Boolean{Boolean: v}
	case int32:
		literal.LiteralType = &Query_Literal_I32{I32: v}
	case int64:
		literal.LiteralType = &Query_Literal_I64{I64: v}
	case uint32:
		literal.LiteralType = &Query_Literal_Ui32{Ui32: v}
	case uint64:
		literal.LiteralType = &Query_Literal_Ui64{Ui64: v}
	case float32:
		literal.LiteralType = &Query_Literal_F32{F32: v}
	case float64:
		literal.LiteralType = &Query_Literal_F64{F64: v}
	// Is this necesary given int64?
	case Timestamp:
		literal.LiteralType = &Query_Literal_Timestamp{Timestamp: int64(v)}
	}
	return literal
}

func makeDpmLiteral(literal *models.LiteralField) *Query_Literal {
	if values, ok := literal.Value.([]interface{}); ok {
		listValues := make([]*Query_Literal, len(values))
		for i, val := range values {
			listValues[i] = makeLiteral(val)
		}
		return &Query_Literal{
			LiteralType: &Query_Literal_List_{
				List: &Query_Literal_List{Values: listValues},
			},
		}
	}
	return makeLiteral(literal.Value)
}

func makeDpmFieldReference(field *models.FieldExpr) *Query_FieldReference {
	fieldName := field.Operands()[0]

	return &Query_FieldReference{
		FieldName: fieldName.(models.FieldExpr).Name,
	}
}

var AGGREGATE_OPERATOR_MAP = map[string]Query_AggregateExpression_AggregateOperator{
	"min":           Query_AggregateExpression_MIN,
	"max":           Query_AggregateExpression_MAX,
	"sum":           Query_AggregateExpression_SUM,
	"count":         Query_AggregateExpression_COUNT,
	"countDistinct": Query_AggregateExpression_COUNT_DISTINCT,
	"avg":           Query_AggregateExpression_MEAN,
	"avgDistinct":   Query_AggregateExpression_MEAN_DISTINCT,
}

var PROJECTION_OPERATOR_MAP = map[string]Query_DerivedExpression_ProjectionOperator{
	"day":         Query_DerivedExpression_DAY,
	"month":       Query_DerivedExpression_MONTH,
	"year":        Query_DerivedExpression_YEAR,
	"hour":        Query_DerivedExpression_HOUR,
	"minute":      Query_DerivedExpression_MINUTE,
	"second":      Query_DerivedExpression_SECOND,
	"millisecond": Query_DerivedExpression_MILLISECOND,
}

func makeDpmAggregateExpression(aggExpr *models.AggregateFieldExpr) *Query_AggregateExpression {
	baseFieldExpr, ok := aggExpr.Operands()[0].(models.FieldExpr)
	if !ok {
		// Handle the case where the type assertion fails
		panic("Expected baseField to be of type models.FieldExpr")
	}

	baseDpmExpr := makeDpmExpression(baseFieldExpr)
	aggOp := aggExpr.Operator()

	dpmAggOp, ok := AGGREGATE_OPERATOR_MAP[string(aggOp)]
	if !ok {
		panic(fmt.Sprintf("Unsupported aggregate operation '%s'", aggOp))
	}

	return &Query_AggregateExpression{
		Argument: baseDpmExpr,
		Op:       dpmAggOp,
	}
}

func makeDpmDerivedExpression(derivedField *models.DerivedField) *Query_DerivedExpression {
	baseField := derivedField.Operands()[0].(models.FieldExpr)
	baseDpmExpr := makeDpmExpression(baseField)
	projectionOp := derivedField.Operator()

	dpmProjectionOp, ok := PROJECTION_OPERATOR_MAP[string(projectionOp)]
	if !ok {
		// Handle unsupported projection operation
		panic(fmt.Sprintf("Unsupported projection operation '%s'", projectionOp))
	}

	return &Query_DerivedExpression{
		Argument: baseDpmExpr,
		Op:       dpmProjectionOp,
	}
}

func makeDpmExpression(field models.Expr) *Query_Expression {
	switch f := field.(type) {
	case *models.LiteralField:
		return &Query_Expression{ExType: &Query_Expression_Literal{Literal: makeDpmLiteral(f)}}
	case *models.AggregateFieldExpr:
		return &Query_Expression{ExType: &Query_Expression_Aggregate{Aggregate: makeDpmAggregateExpression(f)}}
	case *models.DerivedField:
		return &Query_Expression{ExType: &Query_Expression_Derived{Derived: makeDpmDerivedExpression(f)}}
	default:
		if field.Operator() != "ident" {
			// Handle unexpected field expression
			panic(fmt.Sprintf("Unexpected field expression '%v'", field))
		}
		return &Query_Expression{ExType: &Query_Expression_Field{Field: makeDpmFieldReference(f.(*models.FieldExpr))}}
	}
}

// Implement contents of dpm_agent_client.py here

/* func (c *DpmAgentServiceClient) Compile(query *Query) (string, error) {
	// Implementation using c.grpcClient
}

func (c *DpmAgentServiceClient) Execute(query *Query) ([]map[string]interface{}, error) {
	// Implementation using c.grpcClient
} */

func (a *authCreds) GetRequestMetadata(context.Context, ...string) (map[string]string, error) {
	return map[string]string{
		"dpm-auth-token": a.token,
	}, nil
}

func (a *authCreds) RequireTransportSecurity() bool {
	return true
}

func MakeClient(dpmAgentAddress, dpmAuthToken string) (*DpmAgentServiceClient, error) {
	u, err := url.Parse(dpmAgentAddress)
	if err != nil {
		log.Fatalf("invalid address: %v", err)
	}

	var opts []grpc.DialOption
	if u.Scheme == "https" {
		creds := credentials.NewClientTLSFromCert(nil, "")
		opts = append(opts, grpc.WithTransportCredentials(creds))
	} else {
		opts = append(opts, grpc.WithInsecure())
	}
	opts = append(opts, grpc.WithPerRPCCredentials(&authCreds{token: dpmAuthToken}))

	conn, err := grpc.Dial(dpmAgentAddress, opts...)
	if err != nil {
		return nil, err
	}
	//_ = makeDpmExpression(&models.LiteralField{})
	grpcClient := NewDpmAgentClient(conn)
	return &DpmAgentServiceClient{Client: grpcClient, DpmAuthToken: dpmAuthToken}, nil
}
