namespace Dpm
{
	public record Operator
	{
		public record Identity(IdentityOperatorType Op = IdentityOperatorType.identity) : Operator();
		public record Unary(UnaryOperatorType Op) : Operator();
		public record Boolean(BooleanOperatorType Op) : Operator();
		public record Arithmetic(ArithmeticOperatorType Op) : Operator();
		public record Aggregate(AggregateOperatorType Op) : Operator();
		public record Date(DateOperatorType Op) : Operator();
		public record Time(TimeOperatorType Op) : Operator();
		public record Projection(ProjectionOperatorType Op) : Operator();

		private Operator() { }
	}

	public enum IdentityOperatorType
	{
		identity
	}

	public enum UnaryOperatorType
	{
		isNull,
		isNotNull
	}

	public enum BooleanOperatorType
	{
		eq,
		neq,
		gt,
		gte,
		lt,
		lte,
		and,
		or,
		not,
		like,
		@in,
		inPast,
		hasAny,
		hasAll
	}

	public enum ArithmeticOperatorType
	{
		plus,
		minus,
		multiply,
		divide
	}


	public enum AggregateOperatorType
	{
		min,
		max,
		count,
		countDistinct,
		avg,
		avgDistinct,
		sum
	}

	public enum DateOperatorType
	{
		day,
		month,
		year
	}

	public enum TimeOperatorType
	{
		hour,
		minute,
		second,
		millisecond
	}

	public enum ProjectionOperatorType
	{
		day,
		dayOfWeek,
		week,
		month,
		year,
		date,
		time,
		hour,
		minute,
		second,
		millisecond
	}

	public enum DateGranularity
	{
		years,
		months,
		weeks,
		days
	}

	public enum TimeGranularity
	{
		hours,
		minutes,
		seconds,
		milliseconds
	}

	public enum DateTimeGranularity
	{
		years,
		months,
		weeks,
		days,
		hours,
		minutes,
		seconds,
		milliseconds
	}
}
