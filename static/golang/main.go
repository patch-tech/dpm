package main

import (
	"fmt"
	"sync"

	"github.com/patch-tech/dpm/backends"
)

// FactsAppEngagementField represents all possible fields for FactsAppEngagement
type FactsAppEngagementFields struct {
	panelistid          backends.StringField
	age                 backends.Field
	gender              backends.StringField
	ethnicity           backends.StringField
	factid              backends.StringField
	devicemakemodel     backends.StringField
	operator            backends.StringField
	startmarket         backends.StringField
	startinbuilding     backends.StringField
	startzipcode        backends.StringField
	starttimestamp      backends.DateTimeField
	startlongitude      backends.Field
	startlatitude       backends.Field
	appname             backends.StringField
	app_title           backends.StringField
	foregroundduration  backends.Field
	foregroundendtime   backends.DateTimeField
	foregroundstarttime backends.DateTimeField
	screenofftime       backends.DateTimeField
	screenonduration    backends.Field
	screenontime        backends.DateTimeField
	visibleduration     backends.Field
	visibleendtime      backends.DateTimeField
	visiblestarttime    backends.DateTimeField
	isp                 backends.StringField
	start_dataconntech  backends.StringField
	year                backends.Field
	month               backends.Field
	day                 backends.Field
}

// FactsAppEngagement is the singleton struct
type FactsAppEngagementStruct struct {
	Table_ *backends.Table
	Fields *FactsAppEngagementFields
}

var (
	instance *FactsAppEngagementStruct
	once     sync.Once
)

// convertToFactsAppEngagementFields converts a FactsAppEngagementFields instance to a slice of Expr
func convertToFactsAppEngagementFields(faf *FactsAppEngagementFields) []backends.Expr {
	return []backends.Expr{
		&faf.panelistid,
		&faf.age,
		&faf.gender,
		&faf.ethnicity,
		&faf.factid,
		&faf.devicemakemodel,
		&faf.operator,
		&faf.startmarket,
		&faf.startinbuilding,
		&faf.startzipcode,
		&faf.starttimestamp,
		&faf.startlongitude,
		&faf.startlatitude,
		&faf.appname,
		&faf.app_title,
		&faf.foregroundduration,
		&faf.foregroundendtime,
		&faf.foregroundstarttime,
		&faf.screenofftime,
		&faf.screenonduration,
		&faf.screenontime,
		&faf.visibleduration,
		&faf.visibleendtime,
		&faf.visiblestarttime,
		&faf.isp,
		&faf.start_dataconntech,
		&faf.year,
		&faf.month,
		&faf.day,
	}
}

// GetInstance returns the singleton instance of FactsAppEngagement
func FactsAppEngagement() *FactsAppEngagementStruct {
	once.Do(func() {
		fields := &FactsAppEngagementFields{
			panelistid:          *backends.NewStringField("PANELISTID"),
			age:                 *backends.NewField("AGE"),
			gender:              *backends.NewStringField("GENDER"),
			ethnicity:           *backends.NewStringField("ETHNICITY"),
			factid:              *backends.NewStringField("FACTID"),
			devicemakemodel:     *backends.NewStringField("DEVICEMAKEMODEL"),
			operator:            *backends.NewStringField("OPERATOR"),
			startmarket:         *backends.NewStringField("STARTMARKET"),
			startinbuilding:     *backends.NewStringField("STARTINBUILDING"),
			startzipcode:        *backends.NewStringField("STARTZIPCODE"),
			starttimestamp:      *backends.NewDateTimeField("STARTTIMESTAMP"),
			startlongitude:      *backends.NewField("STARTLONGITUDE"),
			startlatitude:       *backends.NewField("STARTLATITUDE"),
			appname:             *backends.NewStringField("APPNAME"),
			app_title:           *backends.NewStringField("APP_TITLE"),
			foregroundduration:  *backends.NewField("FOREGROUNDDURATION"),
			foregroundendtime:   *backends.NewDateTimeField("FOREGROUNDENDTIME"),
			foregroundstarttime: *backends.NewDateTimeField("FOREGROUNDSTARTTIME"),
			screenofftime:       *backends.NewDateTimeField("SCREENOFFTIME"),
			screenonduration:    *backends.NewField("SCREENONDURATION"),
			screenontime:        *backends.NewDateTimeField("SCREENONTIME"),
			visibleduration:     *backends.NewField("VISIBLEDURATION"),
			visibleendtime:      *backends.NewDateTimeField("VISIBLEENDTIME"),
			visiblestarttime:    *backends.NewDateTimeField("VISIBLESTARTTIME"),
			isp:                 *backends.NewStringField("ISP"),
			start_dataconntech:  *backends.NewStringField("START_DATACONNTECH"),
			year:                *backends.NewField("YEAR"),
			month:               *backends.NewField("MONTH"),
			day:                 *backends.NewField("DAY"),
		}
		newTable, err := backends.NewTable(
			"4c5a11b5-e302-4c76-8ae0-e254c19bb581",
			"test-snowflake",
			"0.1.0",
			"FACTS_APP_ENGAGEMENT",
			convertToFactsAppEngagementFields(fields),
			nil,
			"https://example.snowflakecomputing.com",
			nil,
			nil,
			nil,
			1,
		)
		if err != nil {
			// SHould this crash or log?
			println(fmt.Sprintf("error creating NewTable: %v", err))
		}
		instance = &FactsAppEngagementStruct{
			Fields: fields,
			Table_: newTable,
		}
	})
	return instance
}

// Select is a method on FactsAppEngagement to perform selection operations
func (f *FactsAppEngagementStruct) Select(selection ...interface{}) *backends.Table {
	// Implement selection logic
	// This might involve modifying the Table_ property or creating a new Table
	return f.Table_.Select(selection...)
}

func Select(selection ...interface{}) *backends.Table {
	return FactsAppEngagement().Select(selection...)
}

func main() {
	fields := FactsAppEngagement().Fields

	query := Select(fields.app_title.WithAlias("App_Name"),
		fields.foregroundduration.Avg().WithAlias("Avg_Time_in_App"),
		fields.panelistid.CountDistinct().WithAlias("User_Count"),
		fields.starttimestamp.Day().WithAlias("Day_of_Week")).
		Filter(fields.app_title.Like("%Chime%").
			And(fields.startmarket.Like("%Wilmington%"))).
		OrderBy(
			backends.Ordering{
				Field:     "User_Count",
				Direction: "DESC",
			}).
		Limit(10)

	compiled, err := query.Compile()

	if err != nil {
		println(fmt.Sprintf("error compiling query: %v", err))
	}

	println(fmt.Sprintf("%v", compiled))

	executed, err := query.Execute()

	if err != nil {
		println(fmt.Sprintf("error executing query: %v", err))
	}

	println(fmt.Sprintf("%v", executed))

	// Calls using static client
	//simpleSelect(table)
	//selectWithAlias(table)
	//selectWithAgg(table)
}

func selectWithAgg(table *backends.Table) {
	query := table.Select(
		backends.NewStringField("APP_TITLE").WithAlias("App_Name"),
		backends.NewField("FOREGROUNDDURATION").Avg().WithAlias("Avg_Time_in_App"),
		backends.NewStringField("PANELISTID").CountDistinct().WithAlias("User_Count"),
		backends.NewDateTimeField("STARTTIMESTAMP").Day().WithAlias("Day_of_Week")).
		Filter(backends.NewStringField("APP_TITLE").Like("%Chime%").
			And(backends.NewStringField("STARTMARKET").Like("%Wilmington%"))).
		OrderBy(
			backends.Ordering{
				Field:     "User_Count",
				Direction: "DESC",
			}).
		Limit(10)

	compiled, err := query.Compile()

	if err != nil {
		println(fmt.Sprintf("error compiling query: %v", err))
	}

	println(fmt.Sprintf("%v", compiled))

	executed, err := query.Execute()

	if err != nil {
		println(fmt.Sprintf("error executing query: %v", err))
	}

	println(fmt.Sprintf("%v", executed))
}

func selectWithAlias(table *backends.Table) {
	compiled, err := table.Select(
		backends.NewField("APP_TITLE").WithAlias("titulo"),
	).Limit(1).Execute()

	if err != nil {
		println(fmt.Sprintf("error executing query: %v", err))
	}

	println(fmt.Sprintf("%v", compiled))
}

func simpleSelect(table *backends.Table) {
	compiled, err := table.Select(
		backends.NewField("APP_TITLE"),
		backends.NewField("FOREGROUNDDURATION"),
		backends.NewField("PANELISTID"),
		backends.NewField("STARTTIMESTAMP"),
		backends.NewField("STARTMARKET"),
	).Limit(10).Execute()

	if err != nil {
		println(fmt.Sprintf("error executing query: %v", err))
	}

	println(fmt.Sprintf("%v", compiled))
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
