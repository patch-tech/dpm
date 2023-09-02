namespace Dpm
{
  public class StringField : Field<string>
  {
    public StringField(string name) : base(name) { }

    /// <summary>
    /// Returns a boolean expression for a string `like` check.
    /// <see href="https://en.wikibooks.org/wiki/Structured_Query_Language/Like_Predicate#LIKE" />
    /// E.g.,
    /// <c>
    /// var query = MyTable
    ///   .Select(name, price)
    ///   .Filter(name.Like("%shirt%"))
    ///   .Limit(10);
    /// </c>
    /// </summary>
    public BinaryBooleanFieldExpr Like(string pattern)
    {
      return new BinaryBooleanFieldExpr(this, BooleanOperatorType.like, new LiteralField<string>(pattern));
    }
  }
}
