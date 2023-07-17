namespace Domain.Classes;

public sealed class TranslationField
{
  public readonly string BindProperty;
  public readonly string Column;

  public readonly IDictionary<string, string> Values;

  public TranslationField(string column, string bindProperty, IDictionary<string, string> values)
  {
    Values = values;
    Column = column;
    BindProperty = bindProperty;
  }
}
