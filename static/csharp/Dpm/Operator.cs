using System;
namespace dpm
{
	public class Operator
	{
		public const string ident = "ident";

		public readonly string Name = ident;
		public Operator(string? name_ = null)
		{
			if (name_ != null)
			{
				Name = name_;
			}
		}
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
		inPast
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
		month,
		year,
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
		millis

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
		millis

	}
}
