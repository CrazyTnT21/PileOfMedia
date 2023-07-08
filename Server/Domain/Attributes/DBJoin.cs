using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;
using System.Reflection;

namespace Domain.Attributes;

[AttributeUsage(AttributeTargets.Property)]
//This Attribute tells the sql generator to join the specified table
public sealed class DBJoinAttribute : Attribute
{
  public readonly string Table;
  public readonly string Column;
  public readonly string JoinColumn;
  public DBJoinAttribute(string table, string column, string joinColumn)
  {
    JoinColumn = joinColumn;
    Table = table;
    Column = column;
  }

  public DBJoinAttribute(Type type, string joinColumn)
  {
    JoinColumn = joinColumn;

    var att = type.GetCustomAttribute<TableAttribute>();
    if (att is null)
      throw new Exception($"Class '{nameof(type)}' is missing the attribute '{nameof(TableAttribute)}'");

    Table = att.Name;
    var properties = type.GetProperties();
    for (int i = 0; i < properties.Length; i++)
    {
      var prop = properties[i].GetCustomAttribute<KeyAttribute>();
      if (prop is not null)
      {
        var propColumn = properties[i].GetCustomAttribute<ColumnAttribute>();
        if (propColumn is not null && propColumn.Order <= 0)
        {
          if (propColumn.Name is null)
          {
            Column = properties[i].Name;
            return;
          }

          Column = propColumn.Name;
          return;
        }
      }
    }

    throw new Exception($"Class '{nameof(type)}' is missing the attribute '{nameof(KeyAttribute)}'");
  }
}
