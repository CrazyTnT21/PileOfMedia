using System.Reflection;
using System.Text;
using Domain.Enums;
using Domain.ValueObjects;
using Npgsql;

namespace Application.DBMapping;

public sealed class Select<T> where T : new()
{
  private uint Limit;

  private readonly List<Condition> Wheres = new();
  private readonly List<(string Alias, string Column, Order Order)> Orders = new();
  private readonly List<Join> Joins = new();
  private readonly List<QueryProperty> Columns = new();

  private static readonly Dictionary<Type, QueryMapping> Queries = new();
  private readonly QueryMapping Current;

  public Select()
  {
    Current = Queries.ContainsKey(typeof(T)) ? Queries[typeof(T)] : CreateQuery(typeof(T));
  }

  public Select(Type type)
  {
    Current = Queries.ContainsKey(type) ? Queries[type] : CreateQuery(type);
  }

  public Select(string table, bool escape = false)
  {
    QueryMapping currentMapping = new QueryMapping(table, Array.Empty<QueryProperty>(),
                                                   Array.Empty<(Join Join, string? PreviousAlias)>(),
                                                   Array.Empty<JoinProjection>());
    currentMapping.Escape = escape;
    Current = currentMapping;
  }

  public Select<T> Projection(string property)
  {
    //TODO: Implement
    return this;
  }
  public Select<T> Join(string property, Join join)
  {
    //TODO: Implement
    return this;
  }
  public Select<T> Join(Join join)
  {
    //TODO: Implement
    return this;
  }

  private int FindPosition(string property)
  {
    var properties = typeof(T).GetProperties();
    for (int i = 0; i < properties.Length; i++)
    {
      if (properties[i].Name == property)
      {
        return i;
      }
    }

    return -1;
  }

  public Select<T> OrderBy(string column, Order order = Order.Ascending)
  {
    Orders.Add((Current.Alias, column, order));
    return this;
  }

  public Select<T> Column(string column, string? property = null)
  {
    var position = FindPosition(property ?? column);

    if (position < 0)
      throw new Exception($"Property {property ?? column} does not exist on {typeof(T)}");

    for (int i = 0; i < Current.Columns.Length; i++)
      if (Current.Columns[i].Column == column)
        throw new Exception(
          $"Column {column} already exists in the Select. Querying the same column twice is a bad idea");

    QueryProperty columnProperty = new QueryProperty(new[] { (uint)position }, column, Current.Alias);

    Columns.Add(columnProperty);
    return this;
  }

  public Select<T> Column(TableColumn tableColumn, string? property = null)
  {
    for (int i = 0; i < Current.Joins.Length; i++)
    {
      if (Current.Joins[i].originalAlias == tableColumn.Table)
      {
        // Columns.Add(tableColumn);
        return this;
      }
    }

    for (int i = 0; i < Joins.Count; i++)
    {
      if (Joins[i].Alias == tableColumn.Table)
      {
        //  Columns.Add(tableColumn);
        return this;
      }
    }

    throw new Exception($"Alias '{tableColumn.Table}' in Column not found");
  }

  public Select<T> OrderBy(TableColumn tableColumn, Order order)
  {
    for (int i = 0; i < Current.Joins.Length; i++)
    {
      if (Current.Joins[i].originalAlias == tableColumn.Table)
      {
        Orders.Add((Current.Joins[i].Join.Alias!, tableColumn.Column, order));
        return this;
      }
    }

    for (int i = 0; i < Joins.Count; i++)
    {
      if (Joins[i].Alias == tableColumn.Table)
      {
        Orders.Add((Current.Joins[i].Join.Alias!, tableColumn.Column, order));
        return this;
      }
    }

    throw new Exception($"Alias '{tableColumn.Table}' in OrderBy not found");
  }

  public Select<T> Take(uint count)
  {
    Limit = count;
    return this;
  }

  public Select<T> Where(string column, object? value, Comparison comparison = Comparison.Equals)
  {
    Wheres.Add(new Condition(new TableColumn(Current.Alias, column), value, null, comparison));
    return this;
  }

  /// <exception cref="Exception">TODO: Implement => Throws an exception if the given table is not unique, in which case an alias must be used</exception>
  public Select<T> Where(TableColumn tableColumn, object? value, Comparison comparison = Comparison.Equals)
  {
    for (int i = 0; i < Current.Joins.Length; i++)
    {
      if (Current.Joins[i].originalAlias == tableColumn.Table)
      {
        Wheres.Add(new Condition(new TableColumn(Current.Joins[i].Join.Alias!, tableColumn.Column), value, null,
                                 comparison));
        return this;
      }
    }

    for (int i = 0; i < Joins.Count; i++)
    {
      if (Joins[i].Alias == tableColumn.Table)
      {
        Wheres.Add(new Condition(new TableColumn(Joins[i].Alias!, tableColumn.Column), value, null,
                                 comparison));
        return this;
      }
    }

    throw new Exception($"Alias '{tableColumn.Table}' in Where not found");
  }

  public Select<T> Where(string column, TableColumn tableColumn)
  {
    for (int i = 0; i < Current.Joins.Length; i++)
    {
      if (Current.Joins[i].originalAlias == tableColumn.Table)
      {
        Wheres.Add(new Condition(new TableColumn(Current.Alias, column),
                                 new TableColumn(Current.Joins[i].Join.Alias!, tableColumn.Column)));
        return this;
      }
    }

    for (int i = 0; i < Joins.Count; i++)
    {
      if (Joins[i].Alias == tableColumn.Table)
      {
        Wheres.Add(new Condition(new TableColumn(Current.Alias, column),
                                 new TableColumn(Joins[i].Alias!, tableColumn.Column)));
        return this;
      }
    }

    throw new Exception($"Alias '{tableColumn.Table}' in Where not found");
  }

  /// <summary>
  /// join on join. If you intend to use where on the base table use Where(string column, TableColumn tablecolumn)
  /// </summary>
  public Select<T> Where(TableColumn firstTablecolumn, TableColumn tableColumn)
  {
    for (int i = 0; i < Current.Joins.Length; i++)
    {
      if (Current.Joins[i].originalAlias != tableColumn.Table)
        continue;

      for (int j = 0; j < Current.Joins.Length; j++)
      {
        if (Current.Joins[j].originalAlias == firstTablecolumn.Table)
        {
          Wheres.Add(new Condition(new TableColumn(Current.Joins[j].Join.Alias!, firstTablecolumn.Column),
                                   new TableColumn(Current.Joins[i].Join.Alias!, tableColumn.Column)));
          return this;
        }
      }
    }

    for (int i = 0; i < Joins.Count; i++)
    {
      if (Joins[i].Alias != tableColumn.Table)
        continue;

      for (int j = 0; j < Joins.Count; j++)
      {
        if (Joins[j].Alias == firstTablecolumn.Table)
        {
          Wheres.Add(new Condition(new TableColumn(Joins[j].Alias!, firstTablecolumn.Column),
                                   new TableColumn(Joins[i].Alias!, tableColumn.Column)));
          return this;
        }
      }
    }

    throw new Exception($"Alias '{tableColumn.Table}' or '{firstTablecolumn.Table}' in Where not found");
  }

  public Select<T> OverrideValue(object? value, string overrideId)
  {
    bool found = false;
    for (int i = 0; i < Current.Joins.Length; i++)
    {
      for (int j = 0; j < Current.Joins[i].Join.Conditions.Length; j++)
      {
        if (Current.Joins[i].Join.Conditions[j].OverrideId == overrideId)
        {
          Current.Joins[i].Join.Conditions[j].Value = value;
          found = true;
        }
      }
    }

    if (found)
      return this;

    throw new Exception($"Override for overrideId {overrideId} not found");
  }

  private QueryMapping CreateQuery(Type type)
  {
    if (DBMapper.Mappings is null)
      throw new Exception("Mappings are uninitialized. Call SetMappings to initialize mappings");

    IDBMapping? mapping = GetMapping(type);

    if (mapping is null)
      throw new Exception($"Mapping for type {type} missing");

    QueryMapping currentMapping = new QueryMapping(mapping.Table, new QueryProperty[mapping.Columns.Count],
                                                   Array.Empty<(Join Join, string? PreviousAlias)>(),
                                                   new JoinProjection[mapping.JoinProjections.Count]);
    currentMapping.Escape = mapping.Escape;


    for (int i = 0; i < currentMapping.Columns.Length; i++)
    {
      uint[] position = { mapping.Columns[i].PropertyPosition };
      currentMapping.Columns[i] = new QueryProperty(position, mapping.Columns[i].ColumnName, currentMapping.Alias);
    }

    for (int i = 0; i < currentMapping.JoinProjections.Length; i++)
    {
      currentMapping.JoinProjections[i] = mapping.JoinProjections[i];
    }

    AddMapping(currentMapping, new List<uint>(), mapping);
    Queries.Add(type, currentMapping);
    return currentMapping;
  }

  public void AddMapping(QueryMapping currentQuery, List<uint> position, IDBMapping mapping)
  {
    for (int i = 0; i < mapping.Joins.Count; i++)
    {
      JoinDo(currentQuery, mapping.Joins[i], mapping, position);
    }
  }

  public void JoinDo(QueryMapping currentQuery, JoinMapping oldJoin, IDBMapping mapping, List<uint> position)
  {
    Join join = new Join(oldJoin.Join.Table,
                         oldJoin.Join.TableEscape,
                         currentQuery.NextAlias(oldJoin.Join.Table),
                         oldJoin.Join.OnTable,
                         oldJoin.Join.Conditions);

    for (int j = 0; j < join.Conditions.Length; j++)
    {
      if (join.OnTable == join.Conditions[j].TableColumn.Table)
      {
        join.Conditions[j].TableColumn = new TableColumn(currentQuery.Alias, join.Conditions[j].TableColumn.Column);
      }

      if (join.Conditions[j].OtherColumn is not null && join.OnTable == join.Conditions[j].OtherColumn!.Value.Table)
      {
        join.Conditions[j].OtherColumn =
          new TableColumn(currentQuery.Alias, join.Conditions[j].OtherColumn!.Value.Column);
      }

      if (join.Conditions[j].TableColumn.Table == oldJoin.Join.Table ||
          oldJoin.Join.Alias == join.Conditions[j].TableColumn.Table)
      {
        join.Conditions[j].TableColumn = new TableColumn(join.Alias!, join.Conditions[j].TableColumn.Column);
      }

      if (join.Conditions[j].OtherColumn is not null &&
          (join.Conditions[j].OtherColumn!.Value.Table == oldJoin.Join.Table ||
           oldJoin.Join.Alias == join.Conditions[j].OtherColumn!.Value.Table))
      {
        join.Conditions[j].OtherColumn = new TableColumn(join.Alias!, join.Conditions[j].OtherColumn!.Value.Column);
      }
    }


    (Join Join, string? PreviousAlias)[] tempJoins =
      new (Join Join, string? PreviousAlias)[currentQuery.Joins.Length + 1];
    currentQuery.Joins.CopyTo(tempJoins, 0);
    tempJoins[currentQuery.Joins.Length] = (join, oldJoin.Join.Alias);
    currentQuery.Joins = tempJoins;

    var current = new List<uint>();
    current.AddRange(position);
    var projections = new List<QueryProperty>();
    if (oldJoin.Join.Alias is not null)
    {
      for (int j = 0; j < mapping.JoinProjections.Count; j++)
      {
        if (mapping.JoinProjections[j].Alias == oldJoin.Join.Alias)
        {
          var posArray = new uint[current.Count + 1];
          current.CopyTo(posArray);

          posArray[^1] = mapping.JoinProjections[j].PropertyPosition;
          projections.Add(new QueryProperty(posArray, mapping.JoinProjections[j].JoinProperty, join.Alias!));
        }
      }
    }

    QueryProperty[] tempColumns =
      new QueryProperty[currentQuery.Columns.Length + projections.Count];
    currentQuery.Columns.CopyTo(tempColumns, 0);

    for (int j = 0; j < projections.Count; j++)
    {
      tempColumns[currentQuery.Columns.Length + j] = projections[j];
    }

    currentQuery.Columns = tempColumns;

    if (oldJoin.Property is null)
      return;

    current.Add(oldJoin.Property!.Value);

    IDBMapping? map = GetMapping(oldJoin.Join.Table);

    if (map is null)
      return;

    tempColumns = new QueryProperty[currentQuery.Columns.Length + map.Columns.Count];
    currentQuery.Columns.CopyTo(tempColumns, 0);

    for (int j = 0; j < map.Columns.Count; j++)
    {
      var posArray = new uint[current.Count + 1];
      current.CopyTo(posArray);

      posArray[^1] = map.Columns[j].PropertyPosition;
      tempColumns[currentQuery.Columns.Length + j] =
        new QueryProperty(posArray, map.Columns[j].ColumnName, join.Alias!);
    }

    currentQuery.Columns = tempColumns;

    var tempJoinProjections = new JoinProjection[currentQuery.JoinProjections.Length + map.JoinProjections.Count];
    currentQuery.JoinProjections.CopyTo(tempJoinProjections, 0);
    for (int j = 0; j < map.JoinProjections.Count; j++)
    {
      currentQuery.JoinProjections[currentQuery.JoinProjections.Length + j] = map.JoinProjections[j];
    }

    AddMapping(currentQuery, current, map);
  }

  private IDBMapping? GetMapping(string table)
  {
    for (int j = 0; j < DBMapper.Mappings!.Length; j++)
    {
      if (DBMapper.Mappings[j].Table == table)
      {
        return DBMapper.Mappings[j];
      }
    }

    return null;
  }

  private IDBMapping? GetMapping(Type type)
  {
    for (int i = 0; i < DBMapper.Mappings!.Length; i++)
    {
      if (DBMapper.Mappings[i].Type == type)
      {
        return DBMapper.Mappings[i];
      }
    }

    return null;
  }

  private void FormattedFromType(StringBuilder builder, object value, Comparison comparison,
                                 List<NpgsqlParameter> parameters)
  {
    if (value is string)
    {
      if (comparison == Comparison.Like || comparison == Comparison.ILike)
      {
        if (comparison == Comparison.Like)
          builder.Append(" LIKE");
        else
          builder.Append(" ILIKE");
        builder.Append("'%'||");
        parameters.Add(new NpgsqlParameter { Value = value });
        builder.Append('$');
        builder.Append(parameters.Count);
        builder.Append("||'%'");
      }
      else
      {
        builder.Append('=');
        builder.Append('\'');
        parameters.Add(new NpgsqlParameter { Value = value });
        builder.Append('$');
        builder.Append(parameters.Count);
        builder.Append('\'');
      }
    }
    else
    {
      builder.Append('=');
      parameters.Add(new NpgsqlParameter { Value = value });
      builder.Append('$');
      builder.Append(parameters.Count);
    }
  }

  private (string text, List<NpgsqlParameter> parameters) CreateQueryText()
  {
    StringBuilder query = new StringBuilder("SELECT ");
    List<NpgsqlParameter> parameters = new List<NpgsqlParameter>();
    if (Current.Columns.Length > 0)
      query.Append(Current.Columns[0].TableAlias + "." + Current.Columns[0].Column);
    for (int i = 1; i < Current.Columns.Length; i++)
    {
      query.Append(',');
      query.Append(Current.Columns[i].TableAlias);
      query.Append('.');
      query.Append(Current.Columns[i].Column);
    }

    if (Columns.Count > 0)
      query.Append(Columns[0].TableAlias + "." + Columns[0].Column);
    for (int i = 1; i < Columns.Count; i++)
    {
      query.Append(',');
      query.Append(Columns[i].TableAlias);
      query.Append('.');
      query.Append(Columns[i].Column);
    }

    if (Current.Escape)
      query.Append(" FROM \"" + Current.Table + "\" AS " + Current.Alias);
    else
      query.Append(" FROM " + Current.Table + " AS " + Current.Alias);

    for (int i = 0; i < Current.Joins.Length; i++)
    {
      query.Append(" " + Current.Joins[i].Join.CreateJoin(JoinType.Left));
    }

    if (Wheres.Count > 0)
    {
      query.Append(" WHERE ");
      Where(query, Wheres[0], parameters);
    }

    for (int i = 1; i < Wheres.Count; i++)
    {
      query.Append(" AND ");
      Where(query, Wheres[i], parameters);
    }

    if (Limit > 0)
    {
      query.Append(" LIMIT ");
      query.Append(Limit);
    }

    if (Orders.Count > 0)
    {
      query.Append(" ORDER BY ");
      AppendOrder(query, Orders[0]);
    }

    for (int i = 1; i < Orders.Count; i++)
    {
      query.Append(',');
      AppendOrder(query, Orders[i]);
    }

  #if DEBUG
    string result = query.ToString();
    string debugResult = result;
    for (int i = 0; i < parameters.Count; i++)
      debugResult = result.Replace($"${i + 1}", parameters[i].Value?.ToString());
    Console.WriteLine(debugResult);
    return (result, parameters);
  #endif

    return (query.ToString(), parameters);
  }

  private void AppendOrder(StringBuilder builder, (string Alias, string Column, Order Order) order)
  {
    builder.Append(order.Alias);
    builder.Append('.');
    builder.Append(order.Column);
    if (order.Order == Order.Descending)
      builder.Append(" DESC");
  }

  public async Task<T?> UniqueResult(NpgsqlConnection connection)
  {
    var query = CreateQueryText();

    try
    {
      await connection.OpenAsync();

      NpgsqlCommand cmd = new NpgsqlCommand(query.text, connection);
      cmd.Parameters.AddRange(query.parameters.ToArray());
      await cmd.PrepareAsync();
      await using NpgsqlDataReader result = await cmd.ExecuteReaderAsync();

    #if DEBUG
      if (result.Rows > 1)
        throw new Exception("The Query returned more than one row!");
    #endif

      if (!result.HasRows)
        return default;
      await result.ReadAsync();
      T item = new T();
      for (int i = 0; i < Current.Columns.Length; i++)
      {
        if (!result.IsDBNull(i))
          SetValue(item, Current.Columns[i].PropertyPosition, result[i]);
      }

      for (int i = 0; i < Columns.Count; i++)
      {
        if (!result.IsDBNull(i))
          SetValue(item, Columns[i].PropertyPosition, result[Current.Columns.Length + i]);
      }

      return item;
    }
    finally
    {
      await connection.CloseAsync();
    }
  }

  public async IAsyncEnumerable<T> QueryDB(NpgsqlConnection connection)
  {
    var query = CreateQueryText();

    NpgsqlCommand cmd = new NpgsqlCommand(query.text, connection);
    cmd.Parameters.AddRange(query.parameters.ToArray());

    await cmd.PrepareAsync();
    await using NpgsqlDataReader result = await cmd.ExecuteReaderAsync();

    while (await result.ReadAsync())
    {
      T item = new T();
      for (int i = 0; i < Current.Columns.Length; i++)
      {
        if (!result.IsDBNull(i))
          SetValue(item, Current.Columns[i].PropertyPosition, result[i]);
      }

      for (int i = 0; i < Columns.Count; i++)
      {
        if (!result.IsDBNull(Current.Columns.Length + i))
          SetValue(item, Columns[i].PropertyPosition, result[Current.Columns.Length + i]);
      }

      yield return item;
    }
  }

  public async Task<List<T>> List(NpgsqlConnection connection)
  {
    try
    {
      await connection.OpenAsync();

      var result = new List<T>();
      var enumerator = QueryDB(connection).GetAsyncEnumerator();

      while (await enumerator.MoveNextAsync())
      {
        result.Add(enumerator.Current);
      }

      return result;
    }
    finally
    {
      await connection.CloseAsync();
    }
  }

  private void Where(StringBuilder query, Condition condition, List<NpgsqlParameter> parameters)
  {
    query.Append(condition.TableColumn.Combined);

    if (condition.OtherColumn is null)
    {
      if (condition.Value is null)
        query.Append(" is NULL");
      else
        FormattedFromType(query, condition.Value, condition.Operator, parameters);
    }
    else
    {
      query.Append('=');
      query.Append(condition.OtherColumn.Value.Combined);
    }
  }

  private static void SetValue(T item, uint[] ordinal, object value)
  {
    object tempObject = item!;
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
      value = DateOnly.FromDateTime((DateTime)value);
    }
    else
    {
      if (property.PropertyType.IsEnum)
      {
        value = Enum.Parse(property.PropertyType, (string)value);
      }
      else
      {
        Type? type = Nullable.GetUnderlyingType(property.PropertyType);
        if (type is not null && type.IsEnum)
          value = Enum.Parse(type, (string)value);
      }
    }

    property.SetValue(tempObject, value);
  }
}
