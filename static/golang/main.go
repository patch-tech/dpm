package main

import (
	"fmt"

	"github.com/patch-tech/dpm/backends"
)

func main() {
	table, err := backends.NewTable(
		"4c5a11b5-e302-4c76-8ae0-e254c19bb581",
		"test-snowflake",
		"0.1.0",
		"FACTS_APP_ENGAGEMENT",
		nil,
		nil,
		"https://example.snowflakecomputing.com",
		nil,
		nil,
		nil,
		1,
	)

	if err != nil {
		println(fmt.Sprintf("error creating NewTable: %v", err))
	}

	compiled, err := table.Select(backends.NewField("APP_TITLE")).Limit(1).Compile()

	if err != nil {
		println(fmt.Sprintf("error executing query: %v", err))
	}

	println(compiled)
}

// old test code
/*
import (
	"context"
	"log"
	"time"

	pb "github.com/patch-tech/dpm/backends/dpm_agent"
)

func main() {
	address := "https://agent.dpm.sh:443"
	authToken := "NOPE"
	c := cl.MakeClient(address, authToken)
	// Use the client...
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*10)
	defer cancel()

	//table_alias := "t0"
	limit := uint64(10)

	// Define the query
	q := &pb.Query{
		// Define the query fields here
		SelectFrom: "FACTS_APP_ENGAGEMENT",
		//TableAlias: &table_alias,
		Select: []*pb.Query_SelectExpression{
			{
				Argument: &pb.Query_Expression{
					ExType: &pb.Query_Expression_Field{
						Field: &pb.Query_FieldReference{
							FieldName: "APP_TITLE",
							//TableName: &table_alias,
						},
					},
				},
			},
		},
		ClientVersion: &pb.ClientVersion{
			Client:         pb.ClientVersion_PYTHON,
			DatasetVersion: "0.1.0",
		},
		Limit: &limit,
		Id: &pb.Query_Id{
			IdType: &pb.Query_Id_PackageId{
				PackageId: "4c5a11b5-e302-4c76-8ae0-e254c19bb581",
			},
		},
	}

	// Call the ExecuteQuery method
	r, err := c.ExecuteQuery(ctx, q)
	if err != nil {
		log.Fatalf("could not execute query: %v", err)
	}
	log.Printf("Query Result: %v", r)
}
*/
