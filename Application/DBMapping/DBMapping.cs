using System.Reflection;
using Domain.ValueObjects;

namespace Application.DBMapping;

public struct Column
{
  public uint PropertyPosition;
  public string ColumnName;

  public Column(uint position, string columnName)
  {
    PropertyPosition = position;
    ColumnName = columnName;
  }
}

public sealed class DBMapping<T> : IDBMapping
{
  public bool Escape { get; set; }
  public string Table { get; }
  public Type Type { get; } = typeof(T);
  public List<Column> Columns { get; set; } = new List<Column>();
  public List<JoinMapping> Joins { get; set; } = new List<JoinMapping>();
  public List<JoinProjection> JoinProjections { get; set; } = new List<JoinProjection>();
  public List<IDBMapping> ArrayJoins { get; set; } = new List<IDBMapping>();

  private PropertyInfo[] properties;

  public DBMapping(bool escape = false)
  {
    Escape = escape;
    Table = typeof(T).Name;
    properties = typeof(T).GetProperties();
  }

  public DBMapping(string table, bool escape = false)
  {
    Escape = escape;
    Table = table;
    properties = typeof(T).GetProperties();
  }

  public DBMapping<T> Column(string property, string column)
  {
    int position = PropertyPosition(property);
    if (position < 0)
    {
      throw new Exception($"Property '{property}' does not exist on type '{typeof(T)}'");
    }

    for (int i = 0; i < Columns.Count; i++)
    {
      if (Columns[i].PropertyPosition == position)
        throw new Exception($"Mapping for '{typeof(T)}' already contains Property '{property}'");
    }

    Columns.Add(new Column((uint)position, column));

    return this;
  }

  public DBMapping<T> Join(string? property, Join join)
  {
    int? position = null;
    if (property is not null)
    {
      position = PropertyPosition(property);
      if (position < 0)
      {
        throw new Exception($"Property '{property}' does not exist on type '{typeof(T)}'");
      }

      for (int i = 0; i < Joins.Count; i++)
      {
        if (Joins[i].Property == position)
          throw new Exception($"Property '{property}' already exists for Join on type '{typeof(T)}'");
      }
    }

    if (join.Alias is not null)
    {
      for (int i = 0; i < Joins.Count; i++)
      {
        if (Joins[i].Join.Alias == join.Alias)
          throw new Exception($"Alias '{join.Alias}' already exists on type '{typeof(T)}'");
      }
    }

    Joins.Add(new JoinMapping((uint?)position, join));

    return this;
  }

  public DBMapping<T> ArrayJoin(string property, params Condition[] conditions)
  {
    // int? position = PropertyPosition(property);
    // if (position < 0)
    // {
    //   throw new Exception($"Property '{property}' does not exist on type '{typeof(T)}'");
    // }
    //
    // ArrayJoins.Add((uint)position);
    return this;
  }

  public DBMapping<T> Join(Join join)
  {
    Join(null, join);
    return this;
  }

  public DBMapping<T> JoinProjection(string property, string alias, string joinProperty)
  {
    int position = PropertyPosition(property);
    if (position < 0)
    {
      throw new Exception($"Property '{property}' does not exist on type '{typeof(T)}'");
    }

    for (int i = 0; i < JoinProjections.Count; i++)
    {
      if (JoinProjections[i].PropertyPosition == position)
        throw new Exception($"Property '{property}' already exists for Join on type '{typeof(T)}'");
    }

    JoinProjections.Add(new JoinProjection
                          { PropertyPosition = (uint)position, Alias = alias, JoinProperty = joinProperty });

    return this;
  }

  private int PropertyPosition(string property)
  {
    for (int i = 0; i < properties.Length; i++)
    {
      if (properties[i].Name == property)
        return i;
    }

    return -1;
  }
}

public interface IDBMapping
{
  public bool Escape { get; set; }
  public List<Column> Columns { get; set; }
  public List<JoinMapping> Joins { get; }
  public List<JoinProjection> JoinProjections { get; set; }
  public List<IDBMapping> ArrayJoins { get; set; }
  public string Table { get; }
  public Type Type { get; }
}
