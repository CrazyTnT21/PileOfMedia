using System.ComponentModel.DataAnnotations.Schema;
using System.Reflection;

namespace Domain.Attributes;

[AttributeUsage(AttributeTargets.Property)]
public sealed class DBManyAttribute<T> : Attribute
{
  public string Column;
  public string OtherColumn;
  public string Table;

  public DBManyAttribute(string table, string otherColumn)
  {
    Table = table;
    OtherColumn = otherColumn;

    var att = typeof(T).GetCustomAttribute<TableAttribute>();
    if (att is null)
      throw new Exception($"Class '{nameof(T)}' is missing the attribute '{nameof(TableAttribute)}'");

    Table = att.Name;
  }

  public DBManyAttribute(string table, string otherColumn, string column)
  {
    Table = table;
    OtherColumn = otherColumn;
    Column = column;
  }
}
