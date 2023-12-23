namespace Domain.Error;

public class ValueTooLongErrorMessage : StringErrorMessage
{
  public ValueTooLongErrorMessage(string field, int max, int actual) : base(GetErrorText(field, max, actual))
  {
  }

  private static string GetErrorText(string field, int max, int actual)
  {
    return $"Value for {field} was of length {actual}, while only {max} characters are allowed";
  }
}
