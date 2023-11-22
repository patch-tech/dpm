package dpm_agent

import (
	"context"
	"encoding/json"
	"fmt"
	"net/url"

	"github.com/patch-tech/dpm/models"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
	"google.golang.org/grpc/credentials/insecure"
	"google.golang.org/grpc/metadata"
)

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

func makeDpmGroupByExpression(field models.Expr) *Query_GroupByExpression {
	switch f := field.(type) {
	case *models.DerivedField:
		return &Query_GroupByExpression{
			ExType: &Query_GroupByExpression_Derived{
				Derived: makeDpmDerivedExpression(f),
			},
		}
	default:
		if field.Operator() != "ident" {
			// Handle unexpected field expression
			panic(fmt.Sprintf("Unexpected field expression in groupBy: '%v'", field))
		}
		return &Query_GroupByExpression{
			ExType: &Query_GroupByExpression_Field{
				Field: makeDpmFieldReference(field.(*models.FieldExpr)),
			},
		}
	}
}

func makeDpmSelectExpression(field *models.FieldExpr) *Query_SelectExpression {
	selectExpr := &Query_SelectExpression{
		Argument: makeDpmExpression(field),
	}

	if field.Alias != nil {
		selectExpr.Alias = field.Alias
	}

	return selectExpr
}

var BOOLEAN_OPERATOR_MAP = map[string]Query_BooleanExpression_BooleanOperator{
	"and":       Query_BooleanExpression_AND,
	"or":        Query_BooleanExpression_OR,
	"eq":        Query_BooleanExpression_EQ,
	"neq":       Query_BooleanExpression_NEQ,
	"gt":        Query_BooleanExpression_GT,
	"gte":       Query_BooleanExpression_GTE,
	"lt":        Query_BooleanExpression_LT,
	"lte":       Query_BooleanExpression_LTE,
	"like":      Query_BooleanExpression_LIKE,
	"between":   Query_BooleanExpression_BETWEEN,
	"in":        Query_BooleanExpression_IN,
	"isNull":    Query_BooleanExpression_IS_NULL,
	"isNotNull": Query_BooleanExpression_IS_NOT_NULL,
	"hasAny":    Query_BooleanExpression_HAS_ANY,
	"hasAll":    Query_BooleanExpression_HAS_ALL,
	// "not":       //
	// "inPast":    //
}

func makeDpmBooleanExpression(filter models.Expr) *Query_BooleanExpression {
	booleanFilter, ok := filter.(*models.BooleanFieldExpr)
	if !ok {
		panic("Expected *models.BooleanFieldExpr")
	}

	op := booleanFilter.Op // Assuming Op is accessible directly or via a method

	var args []*Query_Expression

	if op == "and" || op == "or" {
		for _, operand := range booleanFilter.Operands() {
			args = append(args, makeDpmExpression(operand))
		}
		return &Query_BooleanExpression{
			Op:        BOOLEAN_OPERATOR_MAP[string(op)],
			Arguments: args,
		}
	}

	// Handle other boolean operators
	dpmBooleanOp, ok := BOOLEAN_OPERATOR_MAP[string(op)]
	if !ok {
		panic(fmt.Sprintf("Unhandled boolean operator '%v'", op))
	}

	for _, expr := range filter.Operands() {
		args = append(args, makeDpmExpression(expr))
	}

	return &Query_BooleanExpression{
		Op:        dpmBooleanOp,
		Arguments: args,
	}
}

func makeDpmOrderByExpression(ordering *models.Ordering) *Query_OrderByExpression {
	fieldExpr := ordering.Field
	direction := ordering.Direction

	var dpmDirection Query_OrderByExpression_Direction
	if direction == "ASC" {
		dpmDirection = Query_OrderByExpression_ASC
	} else {
		dpmDirection = Query_OrderByExpression_DESC
	}

	return &Query_OrderByExpression{
		Argument:  makeDpmExpression(fieldExpr),
		Direction: &dpmDirection,
	}
}

type authCreds struct {
	token string
}

type DpmAgentServiceClient struct {
	Client       DpmAgentClient
	DpmAuthToken string
}

// NewDpmAgentServiceClient creates a new instance of DpmAgentServiceClient.
func NewDpmAgentServiceClient(client DpmAgentClient, dpmAuthToken string) *DpmAgentServiceClient {
	return &DpmAgentServiceClient{
		Client:       client,
		DpmAuthToken: dpmAuthToken,
	}
}

// authCreds implements credentials.PerRPCCredentials for setting the auth token.
func (a *authCreds) GetRequestMetadata(ctx context.Context, uri ...string) (map[string]string, error) {
	return map[string]string{
		"dpm-auth-token": a.token,
	}, nil
}

func (a *authCreds) RequireTransportSecurity() bool {
	return true // or false, depending on whether you require transport security
}

// MakeDpmAgentQuery constructs a DpmAgentQuery based on the provided TableExpression.
func (client *DpmAgentServiceClient) MakeDpmAgentQuery(query *models.Table) (*Query, error) {
	dpmAgentQuery := &Query{
		Id: &Query_Id{
			IdType: &Query_Id_PackageId{
				PackageId: query.PackageID,
			},
		},
		ClientVersion: &ClientVersion{
			Client:         ClientVersion_PYTHON, // or other client type
			CodeVersion:    models.CODE_VERSION,
			DatasetVersion: query.DatasetVersion,
		},
		SelectFrom: query.Name,
	}

	// Handle selections
	if len(query.Selection) > 0 {
		for _, expr := range query.Selection {
			if fieldExpr, ok := (*expr).(*models.FieldExpr); ok {
				selectExpr := makeDpmSelectExpression(fieldExpr)
				dpmAgentQuery.Select = append(dpmAgentQuery.Select, selectExpr)
			} else {
				// Handle error or unexpected types
			}
		}
	}

	// Handle filter expression
	if query.FilterExpr != nil {
		filterExpr := makeDpmBooleanExpression(query.FilterExpr.(*models.BooleanFieldExpr))
		dpmAgentQuery.Filter = filterExpr
	}

	// Handle group by
	if len(query.Selection) > 0 {
		for _, expr := range query.Selection {
			if _, ok := (*expr).(*models.AggregateFieldExpr); !ok {
				if fieldExpr, ok := (*expr).(*models.FieldExpr); ok {
					groupByExpr := makeDpmGroupByExpression(fieldExpr)
					dpmAgentQuery.GroupBy = append(dpmAgentQuery.GroupBy, groupByExpr)
				} else {
					// Handle error or unexpected types
				}
			}
		}
	}

	// Handle order by
	if len(query.Ordering) > 0 {
		for _, ordering := range query.Ordering {
			orderByExpr := makeDpmOrderByExpression(&ordering)
			dpmAgentQuery.OrderBy = append(dpmAgentQuery.OrderBy, orderByExpr)
		}
	}

	// Handle limit
	if query.LimitTo > 0 {
		dpmAgentQuery.Limit = &query.LimitTo
	}

	return dpmAgentQuery, nil
}

// Compile compiles table expression using dpm-agent.
func (client *DpmAgentServiceClient) Compile(query *models.Table) (string, error) {
	dpmAgentQuery, err := client.MakeDpmAgentQuery(query)
	if err != nil {
		return "", err
	}
	trueVal := true
	dpmAgentQuery.DryRun = &trueVal

	response, err := client.Client.ExecuteQuery(context.Background(), dpmAgentQuery, grpcMetadata(client.DpmAuthToken)...)
	if err != nil {
		return "", err
	}

	return response.QueryString, nil
}

// Execute executes table expression using dpm-agent.
func (client *DpmAgentServiceClient) Execute(query *models.Table) ([]map[string]interface{}, error) {
	dpmAgentQuery, err := client.MakeDpmAgentQuery(query)
	if err != nil {
		return nil, err
	}

	response, err := client.Client.ExecuteQuery(context.Background(), dpmAgentQuery, grpcMetadata(client.DpmAuthToken)...)
	if err != nil {
		return nil, err
	}

	var jsonData []map[string]interface{}
	err = json.Unmarshal([]byte(response.JsonData), &jsonData)
	if err != nil {
		return nil, fmt.Errorf("error parsing JSON: %w", err)
	}

	return jsonData, nil
}

// grpcMetadata creates the metadata for the gRPC call.
func grpcMetadata(token string) []grpc.CallOption {
	md := metadata.Pairs("dpm-auth-token", token)
	return []grpc.CallOption{grpc.Header(&md)}
}

// globalClientCache stores gRPC clients keyed by service address.
var globalClientCache = make(map[string]DpmAgentClient)

// MakeClient creates a DpmAgentServiceClient that shares a single gRPC client for a given service address.
func MakeClient(dpmAgentAddress, dpmAuthToken string) (*DpmAgentServiceClient, error) {
	// Check if the client already exists in the cache.
	if client, ok := globalClientCache[dpmAgentAddress]; ok {
		return &DpmAgentServiceClient{Client: client, DpmAuthToken: dpmAuthToken}, nil
	}

	// Parse the service address URL.
	parsedURL, err := url.Parse(dpmAgentAddress)
	if err != nil {
		return nil, fmt.Errorf("invalid DpmAgent address: %v", err)
	}

	// Determine whether to use a secure or insecure channel based on the URL scheme.
	var grpcConn *grpc.ClientConn
	if parsedURL.Scheme == "https" || parsedURL.Port() == "443" {
		grpcConn, err = grpc.Dial(parsedURL.Host, grpc.WithTransportCredentials(credentials.NewTLS(nil)))
	} else {
		grpcConn, err = grpc.Dial(parsedURL.Host, grpc.WithTransportCredentials(insecure.NewCredentials()))
	}
	if err != nil {
		return nil, fmt.Errorf("failed to create gRPC connection: %v", err)
	}

	// Create a new DpmAgentClient.
	dpmAgentClient := NewDpmAgentClient(grpcConn)
	globalClientCache[dpmAgentAddress] = dpmAgentClient

	return &DpmAgentServiceClient{Client: dpmAgentClient, DpmAuthToken: dpmAuthToken}, nil
}
