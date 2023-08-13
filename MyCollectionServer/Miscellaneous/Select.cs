using System.ComponentModel.DataAnnotations.Schema;
using System.Reflection;
using System.Text;
using Domain.Attributes;
using Domain.Enums;
using MySqlConnector;

namespace MyCollectionServer.Miscellaneous;

public sealed class Select<T> where T : new()
{
  private static readonly Dictionary<Type, TableColumn> RegisteredTypes = new();

  private readonly TableColumn Current;

  public uint MaxRecursion = 3;

  private readonly Dictionary<Type, uint> _alreadyDone = new();

  private readonly Dictionary<Key, string> _dynamicColumns = new();

  private uint Limit;

  private Dictionary<string, List<object?>> Wheres = new();
  private Dictionary<string, List<object?>> Likes = new();

  public Select()
  {
    if (!RegisteredTypes.ContainsKey(typeof(T)))
    {
      Current = new TableColumn();
      SetColumns(typeof(T), new uint[] { });
      RegisteredTypes.Add(typeof(T), Current);
    }
    else
    {
      Current = RegisteredTypes[typeof(T)];
    }
  }

  public Select<T> AddDynamicColumn(Key key, string column)
  {
    if (!_dynamicColumns.ContainsKey(key))
    {
      _dynamicColumns.Add(key, column);
    }

    return this;
  }

  public Select<T> Projection()
  {
    //TODO: Implement
    return this;
  }

  public Select<T> Take(uint count)
  {
    Limit = count;
    //TODO: Implement
    return this;
  }

  public Select<T> Where(string property, object? value)
  {
    if (Wheres.ContainsKey(property))
    {
      Wheres[property].Add(value);
    }
    else
    {
      Wheres.Add(property, new List<object?>() { value });
    }

    return this;
  }

  public Select<T> Like(string property, string value)
  {
    //TODO: Implement
    if (Likes.ContainsKey(property))
    {
      Likes[property].Add(value);
    }
    else
    {
      Likes.Add(property, new List<object?>() { value });
    }

    return this;
  }

  public async IAsyncEnumerable<T> QueryDB(MySqlConnection connection)
  {
    TableAttribute? table = typeof(T).GetCustomAttribute<TableAttribute>();

    if (table is null)
      throw new Exception(typeof(TableAttribute) + " is missing on Type " + typeof(T));

    StringBuilder query = new StringBuilder("SELECT ");

    string tableAlias = table.Name[0].ToString();
    Dictionary<string, (Key, string, uint[], Join)> found = new();

    foreach (var value in _dynamicColumns)
    foreach (var column in Current.RuntimeJoin)
      if (value.Key == column.Value.Item1)
        found.Add(column.Key, (value.Key, value.Value, column.Value.Item3, column.Value.Item2));

    if (Current.Columns.Count == 0)
      query.Remove(query.Length - 1, 1);

    uint length = 0;

    foreach (var column in Current.Columns)
    {
      query.Append($"`{column.Key}`.`{column.Value[0].Column}`");
      for (int i = 1; i < column.Value.Count; i++)
      {
        query.Append($",`{column.Key}`.`{column.Value[i].Column}`");
      }

      if (length != Current.Columns.Count - 1)
        query.Append(",");
      length++;
    }

    foreach (var column in found)
    {
      query.Append($",`{column.Key}`.`{column.Value.Item2}`");
    }

    query.Append("FROM " + table.Name + " AS " + tableAlias);

    foreach (var join in Current.Joins)
    {
      query.Append(join.Value.CreateJoin(JoinType.Left));
    }

    foreach (var join in found)
    {
      query.Append(join.Value.Item4.CreateJoin(JoinType.Left));
    }

    foreach (var where in Wheres)
    {
      string[] tables = where.Key.Split(".");
      query.Append(" WHERE ");
      query.Append($"{where.Key} = {where.Value[0]}");

      for (int i = 1; i < where.Value.Count; i++)
        query.Append($" AND {where.Key} = {where.Value[i]}");
    }

    if (Limit > 0)
      query.Append(" LIMIT " + Limit);

    Console.WriteLine(query.ToString());
    MySqlCommand cmd = new MySqlCommand(query.ToString(), connection);

    await cmd.PrepareAsync();
    await using MySqlDataReader result = await cmd.ExecuteReaderAsync();
    uint currentRow = 0;
    while (await result.ReadAsync())
    {
      T item = new T();
      int ordinal = 0;
      foreach (var column in Current.Columns)
      {
        for (int i = 0; i < column.Value.Count; i++)
        {
          if (!result.IsDBNull(ordinal))
            SetValue(column.Value[i].PropertyPosition, result[ordinal], item);
          ordinal++;
        }
      }

      foreach (var column in found)
      {
        if (!result.IsDBNull(ordinal))
          SetValue(column.Value.Item3, result[ordinal], item);
        ordinal++;
      }

      yield return item;
    }
  }

  private void SetValue(uint[] ordinal, object value, object item)
  {
    object tempObject = item;
    for (int i = 0; i < ordinal.Length - 1; i++)
    {
      PropertyInfo[] properties = tempObject.GetType().GetProperties();

      object? propValue = properties[ordinal[i]].GetValue(tempObject);
      if (propValue is null)
      {
        properties[ordinal[i]].SetValue(tempObject, Activator.CreateInstance(properties[ordinal[i]].PropertyType));
      }

      tempObject = properties[ordinal[i]].GetValue(tempObject)!;
    }

    PropertyInfo[] temoObjectProperties = tempObject.GetType().GetProperties();
    PropertyInfo property = temoObjectProperties[ordinal[^1]];
    if (property.PropertyType == typeof(DateOnly?) || property.PropertyType == typeof(DateOnly))
    {
      property.SetValue(tempObject, DateOnly.FromDateTime((DateTime)value));
    }
    else
    {
      property.SetValue(tempObject, value);
    }
  }

  private void SetColumns(Type type, uint[] ordinal)
  {
    TableAttribute? table = type.GetCustomAttribute<TableAttribute>();
    if (table is null)
      return;

    if (_alreadyDone.ContainsKey(type))
    {
      if (_alreadyDone[type] >= MaxRecursion)
        return;
      _alreadyDone[type]++;
    }
    else
    {
      _alreadyDone.Add(type, 0);
    }

    string tableAlias = GetNextAlias(table.Name);
    PropertyInfo[] properties = type.GetProperties();

    for (uint i = 0; i < properties.Length; i++)
    {
      uint[] position = new uint[ordinal.Length + 1];
      ordinal.CopyTo(position, 0);
      position[^1] = i;

      DBJoinAttribute? dbjoin = properties[i].GetCustomAttribute<DBJoinAttribute>();
      if (dbjoin is not null)
      {
        string dbjoinalias = GetNextAlias(dbjoin.Table);

        Join join = new Join(dbjoin.Table, dbjoin.Column, tableAlias, dbjoin.JoinColumn, dbjoinalias);

        DBAssociationAttribute? dbassociation = properties[i].GetCustomAttribute<DBAssociationAttribute>();
        if (dbassociation is not null)
        {
          Current.RuntimeJoin.Add(dbjoinalias, (dbassociation.Key, join, position));
        }
        else
        {
          Current.Joins.Add(dbjoinalias, join);
          SetColumns(properties[i].PropertyType, position);
        }
      }
      else
      {
        ColumnAttribute? column = properties[i].GetCustomAttribute<ColumnAttribute>();
        if (column is not null)
        {
          ColumnMapping columnMapping = new ColumnMapping(position, column.Name ?? properties[i].Name);
          if (Current.Columns.ContainsKey(tableAlias))
          {
            Current.Columns[tableAlias].Add(columnMapping);
          }
          else
          {
            Current.Columns.Add(tableAlias, new List<ColumnMapping>() { columnMapping });
          }
        }
      }
    }
  }

  private string GetNextAlias(string value)
  {
    string alias = value[0].ToString();
    if (!Current.Columns.ContainsKey(alias) && !Current.RuntimeJoin.ContainsKey(alias))
      return alias;

    uint index = 1;

    while (Current.Columns.ContainsKey(alias + index) || Current.RuntimeJoin.ContainsKey(alias + index))
      index++;

    return alias + index;
  }
}
