using System.Reflection;

namespace Domain.Attributes;

[AttributeUsage(AttributeTargets.Class, Inherited = true)]
public sealed class DBTableAttribute<T> : Attribute
{
  public readonly string[] columns;
  public readonly string table;

  public DBTableAttribute(string table)
  {
    this.table = table;
    List<string> Columns = new();
    PropertyInfo[] properties = typeof(T).GetProperties();
    for (int i = 0; i < properties.Length; i++)
    {
      DBColumnAttribute? att = properties[i].GetCustomAttribute<DBColumnAttribute>();
      if (att is not null)
        Columns.Add(att.column);
    }

    columns = Columns.ToArray();
  }
}
