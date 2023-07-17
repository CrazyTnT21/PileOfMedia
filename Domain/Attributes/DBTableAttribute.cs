using System.Reflection;

namespace Domain.Attributes;

[AttributeUsage(AttributeTargets.Class)]
public sealed class DBTableAttribute<T> : Attribute
{
  public readonly string[] columns;
  public readonly string table;

  public DBTableAttribute(string table)
  {
    this.table = table;
    List<string> Columns = new();
    var properties = typeof(T).GetProperties();
    for (int i = 0; i < properties.Length; i++)
    {
      var att = properties[i].GetCustomAttribute<DBColumnAttribute>();
      if (att is not null)
        Columns.Add(att.column);
    }

    columns = Columns.ToArray();
  }
}
