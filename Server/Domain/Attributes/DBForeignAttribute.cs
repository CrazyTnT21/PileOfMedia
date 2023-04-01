namespace Domain.Attributes;

[AttributeUsage(AttributeTargets.Property)]
public sealed class DBForeignAttribute : Attribute
{
  public readonly string Column;
  public readonly string ForeignTable;
  public readonly string ForeignColumn;

  public DBForeignAttribute(string foreignTable, string column, string foreignColumn)
  {
    Column = column;
    ForeignTable = foreignTable;
    ForeignColumn = foreignColumn;
  }
}
