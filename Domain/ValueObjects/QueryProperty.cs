namespace Domain.ValueObjects;

public struct QueryProperty
{
  public readonly uint[] PropertyPosition;
  public readonly string Column;
  public string TableAlias;

  public QueryProperty(uint[] propertyPosition, string column, string tableAlias)
  {
    PropertyPosition = propertyPosition;
    Column = column;
    TableAlias = tableAlias;
  }
}

public sealed class QueryMapping
{
  public QueryProperty[] Columns;
  public (Join Join, string? originalAlias)[] Joins;
  public JoinProjection[] JoinProjections;
  public string Table;
  public string Alias;
  public bool Escape;

  public QueryMapping(string table,
                      QueryProperty[] columns,
                      (Join Join, string? originalAlias)[] joins,
                      JoinProjection[] joinProjections)
  {
    Columns = columns;
    Joins = joins;
    JoinProjections = joinProjections;
    Table = table;
    Alias = NextAlias(Table);
  }

  public string NextAlias(string value)
  {
    string alias = value[0].ToString();
    bool found = false;
    for (int i = 0; i < Joins.Length; i++)
    {
      if (Joins[i].Join.Alias == alias)
      {
        found = true;
        break;
      }
    }

    if (!found && alias != Alias)
      return alias;

    found = true;
    uint index = 1;

    while (found)
    {
      found = false;
      for (int i = 0; i < Joins.Length; i++)
      {
        if (Joins[i].Join.Alias == alias + index && Joins[i].Join.Alias != alias + index)
        {
          found = true;
          break;
        }
      }

      index++;
    }

    return alias + index;
  }
}
