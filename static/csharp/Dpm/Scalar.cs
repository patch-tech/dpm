
namespace dpm
{
  public class Scalar<T>
  {
    private readonly T? value;
    private readonly T[]? values;

    public Scalar(T val_)
    {
      value = val_;
    }

    public Scalar(T[] values_)
    {
      values = values_;
    }

    public bool IsList()
    {
      return values != null;
    }

    public T? Value()
    {
      return value;
    }

    public T[] Values()
    {
      if (values == null)
      {
        return Array.Empty<T>();
      }
      return values;
    }
  }
}
