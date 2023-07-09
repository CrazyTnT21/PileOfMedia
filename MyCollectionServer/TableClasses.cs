using System.ComponentModel;
using System.Reflection;
using System.Text;
using MySqlConnector;

namespace MyCollectionServer;

public sealed class Language
{
  public uint PK;
  [DBColumn("Language")] public string LanguageText { get; set; }
  [DBColumn("Column")] public string ColumnName { get; set; }
};

public sealed class TPublish : PKClass
{
  public uint PK { get; set; }
  public uint FKPublish { get; set; }
}

public sealed class TRelation : PKClass
{
  public uint PK { get; set; }
  public uint FKRelation { get; set; }
}

public sealed class TGenre : PKClass
{
  public uint PK { get; set; }
  public uint FKGenre { get; set; }
}

public sealed class TStatus : PKClass
{
  public uint PK { get; set; }
  public uint FKStatus { get; set; }
}

public sealed class TUserStatus : PKClass
{
  public uint PK { get; set; }
  public uint FKUserStatus { get; set; }
}

public sealed class TTheme : PKClass
{
  public uint PK { get; set; }
  public uint FKTheme { get; set; }
}

public sealed class TRole : PKClass
{
  public uint PK { get; set; }
  public uint FKRole { get; set; }
}

public sealed class TUser : PKClass
{
  [DBColumn("PK")] public uint PK { get; set; }
  [DBColumn("FKPerson")] public uint? FKPerson { get; set; }
  [DBColumn("Name")] public string Name { get; set; }
  [DBColumn("Joined")] public DateTime Joined { get; set; }
  [DBColumn("Description")] public string? Description { get; set; }
  [DBColumn("ImageSource")] public string? ImageSource { get; set; }
  [DBColumn("MangaAverage")] public float? MangaAverage { get; set; }
  [DBColumn("ComicAverage")] public float? ComicAverage { get; set; }
  [DBColumn("TVShowAverage")] public float? TVShowAverage { get; set; }
  [DBColumn("MovieAverage")] public float? MovieAverage { get; set; }
  [DBColumn("AnimeAverage")] public float? AnimeAverage { get; set; }
  [DBColumn("BookAverage")] public float? BookAverage { get; set; }
  [DBColumn("CartoonAverage")] public float? CartoonAverage { get; set; }
  [DBColumn("GameAverage")] public float? GameAverage { get; set; }
};

public sealed class ComicVolume : PKClass
{
  [DBColumn("PK")] public uint PK { get; set; }
  [DBColumn("FKComic")] public Comic Comic { get; set; }
  [DBColumn("FKTitle")] public uint? FKTitle { get; set; }
  [DBColumn("Pages")] public uint? Pages { get; set; }
  [DBColumn("PublishDate")] public DateTime? PublishDate { get; set; }
  [DBColumn("PublishEnd")] public DateTime? PublishEnd { get; set; }
  [DBColumn("ImageSource")] public string? ImageSource { get; set; }
  [DBColumn("AverageScore")] public float? AverageScore { get; set; }
  [DBColumn("Title")] public string? Title { get; set; }
  [DBColumn("Description")] public string? Description { get; set; }

  public LanguageField[]? LanguageFields { get; set; }
};

public sealed class ComicChapter : PKClass
{
  [DBColumn("PK")] public uint PK { get; set; }
  [DBColumn("FKComic")] public Comic Comic { get; set; }
  [DBColumn("FKComicVolume")] public ComicVolume? ComicVolume { get; set; }
  [DBColumn("FKTitle")] public uint? FKTitle { get; set; }
  [DBColumn("Pages")] public uint? Pages { get; set; }
  [DBColumn("PublishDate")] public DateTime PublishDate { get; set; }
  [DBColumn("PublishEnd")] public DateTime PublishEnd { get; set; }
  [DBColumn("ImageSource")] public string? ImageSource { get; set; }
  [DBColumn("AverageScore")] public float? AverageScore { get; set; }
  [DBColumn("Title")] public string? Title { get; set; }
  [DBColumn("Description")] public string? Description { get; set; }

  public LanguageField[]? LanguageFields { get; set; }
};

//Other
public interface PKClass
{
  public uint PK { get; set; }
}

public interface languageFields
{
  public LanguageField[]? LanguageFields { get; set; }
}

public interface Media
{
  public uint? FKName { get; set; }
  public uint? FKDescription { get; set; }

  public string? ImageSource { get; set; }
  public float? AverageScore { get; set; }

  public string? Name { get; set; }
  public string? Description { get; set; }
}

public sealed class Translation
{
  public Translation(string value, string language)
  {
    Value = value;
    Language = language;
  }

  public string Language { get; set; }
  public string Value { get; set; }
}

public sealed class LanguageField
{
  public LanguageField(string column, string bindProperty, params Translation[] values)
  {
    Values = values;
    Column = column;
    BindProperty = bindProperty;
  }

  public Translation[] Values { get; set; }
  public string Column { get; set; }
  public string BindProperty { get; set; }
};

public sealed class Select
{
  private readonly string _table;

  private IDictionary<string, KeyValuePair<string, object?>> _where =
    new Dictionary<string, KeyValuePair<string, object?>>();

  private IDictionary<string, Order> _order = new Dictionary<string, Order>();
  private StringBuilder _query = new StringBuilder();
  private uint limit = 50;
  private IDictionary<string, List<string>> _columns = new Dictionary<string, List<string>>();

  private IDictionary<string, List<KeyValuePair<string, string>>> _aliases =
    new Dictionary<string, List<KeyValuePair<string, string>>>();

  private List<Join> joins = new List<Join>();

  public Select(string table)
  {
    _table = table;
    _columns.Add(_table, new List<string>());
  }

  public Select Join(Join join)
  {
    joins.Add(join);
    return this;
  }

  public Select Alias(string column, string alias)
  {
    _aliases[_table].Add(new KeyValuePair<string, string>(column, alias));

    return this;
  }

  public Select Alias(string table, string column, string alias)
  {
    if (_aliases.ContainsKey(table))
      _aliases[table].Add(new KeyValuePair<string, string>(column, alias));
    else
      _aliases.Add(table, new List<KeyValuePair<string, string>>() { new(column, alias) });

    return this;
  }

  public Select AddColumns(string table, params string[] columns)
  {
    if (_columns.ContainsKey(table))
      _columns[table].AddRange(columns);
    else
      _columns.Add(table, columns.ToList());
    return this;
  }

  public Select AddColumns(string table, List<string> columns)
  {
    if (_columns.ContainsKey(table))
      _columns[table].AddRange(columns);
    else
      _columns.Add(table, columns);

    return this;
  }

  public Select AddColumns(List<string> columns)
  {
    _columns[_table].AddRange(columns);
    return this;
  }

  public Select AddColumns(params string[] columns)
  {
    _columns[_table].AddRange(columns);
    return this;
  }

  public Select AddColumn(string column, string? alias = null)
  {
    _columns[_table].Add(column);
    if (alias is not null)
      _aliases[_table].Add(new KeyValuePair<string, string>(column, alias));
    return this;
  }

  public Select AddColumn(string column, string table, string? alias)
  {
    if (_columns.ContainsKey(table))
      _columns[table].Add(column);
    else
      _columns.Add(table, new List<string>() { column });

    if (alias is not null)
    {
      if (_aliases.ContainsKey(table))
        _aliases[table].Add(new KeyValuePair<string, string>(column, alias));
      else
        _aliases.Add(table, new List<KeyValuePair<string, string>>() { new(column, alias) });
    }

    return this;
  }

  public Select Where(string whereColumn, object? whereValue = null)
  {
    _where.Add(_table, new KeyValuePair<string, object?>(whereColumn, whereValue));
    return this;
  }

  public Select Where(string whereColumn, string table, object? whereValue = null)
  {
    _where.Add(table, new KeyValuePair<string, object?>(whereColumn, whereValue));
    return this;
  }

  public Select Order(string orderColumn, Order order = MyCollectionServer.Order.Ascending)
  {
    _order.Add(orderColumn, order);
    return this;
  }

  public Select Limit(uint limit)
  {
    this.limit = limit;
    return this;
  }

  public Select Join(string table, string column, string onTable, string onColumn, string matchColumn,
    string? alias = null,
    JoinType joinType = JoinType.Left)
  {
    var join = new Join(table, column, onTable, onColumn, alias, joinType);
    joins.Add(join);
    return this;
  }

  public async Task<List<T>> QueryDB<T>(MySqlConnection connection) where T : new()
  {
    List<string> values = new List<string>();
    foreach (var tableColumn in _columns)
    {
      if (_aliases.ContainsKey(tableColumn.Key))
      {
        for (int j = 0; j < tableColumn.Value.Count; j++)
        {
          bool found = false;

          for (int i = 0; i < _aliases[tableColumn.Key].Count; i++)
            if (_aliases[tableColumn.Key][i].Key.Equals(tableColumn.Value[j], StringComparison.OrdinalIgnoreCase))
            {
              values.Add($"`{tableColumn.Key}`.`{tableColumn.Value[j]}` AS `{_aliases[tableColumn.Key][i].Value}`");
              found = true;
            }

          if (!found)
          {
            values.Add($"`{tableColumn.Key}`.`{tableColumn.Value[j]}`");
          }
        }
      }
      else
      {
        for (int j = 0; j < tableColumn.Value.Count; j++)
        {
          values.Add($"`{tableColumn.Key}`.`{tableColumn.Value[j]}`");
        }
      }
    }

    if (values.Count > 0)
      _query.Append($"SELECT {string.Join(',', values)} FROM `{_table}`");
    else
      _query.Append($"SELECT * FROM `{_table}`");

    for (int i = 0; i < joins.Count; i++)
      _query.Append(joins[i].createJoin());
    if (_where.Count > 0)
    {
      List<string> wheres = new List<string>();
      foreach (var item in _where)
      {
        if (item.Value.Value is null)
          wheres.Add($" `{item.Key}`.`{item.Value.Key}` is NULL ");
        else
          wheres.Add($"`{item.Key}`.`{item.Value.Key}` = {item.Value.Value}");
      }

      _query.Append("WHERE " + string.Join(" AND ", wheres));
    }

    if (_order.Count > 0)
    {
      _query.Append("ORDER BY");
      foreach (var item in _order)
        _query.Append(BaseT.OrderByWithout(item.Key, item.Value));
    }

    _query.Append(BaseT.Limit(0, limit));
    Console.WriteLine(_query.ToString());
    return await QueryBase.QueryDB<T>(new MySqlCommand(_query.ToString(), connection));
  }
}

public sealed class Create
{
  private readonly string[] _columns;
  private readonly object[] _values;

  public Create(string[] columns, object[] values)
  {
    _columns = columns;
    _values = values;
  }
}

public sealed class Update
{
}

public sealed class Delete
{
}

public sealed class Join
{
  // public readonly string join;
  public readonly string Table;
  public readonly string Column;
  public readonly string? Alias;
  public readonly string OnTable;
  public readonly string OnColumn;
  public readonly JoinType JoinType;

  public Join(string table, string column, string onTable, string onColumn, string? alias = null,
    JoinType joinType = JoinType.Left)
  {
    Table = table;
    Alias = alias;
    OnColumn = onColumn;
    OnTable = onTable;
    JoinType = joinType;
    Column = column;

    //join = createJoin();
  }

  public static string getJoinType(JoinType type)
  {
    switch (type)
    {
      case JoinType.Inner:
        return "INNER";
      case JoinType.Left:
        return "LEFT";
      case JoinType.Right:
        return "RIGHT";
      default: throw new InvalidEnumArgumentException();
    }
  }

  public string createJoin()
  {
    if (Alias is not null)
      return
        $" {getJoinType(JoinType)} JOIN `{Table}` AS `{Alias}` ON `{OnTable}`.`{OnColumn}` = `{Alias}`.`{Column}` ";
    return $" {getJoinType(JoinType)} JOIN `{Table}` ON `{OnTable}`.`{OnColumn}` = `{Table}`.`{Column}` ";
  }
}

public enum JoinType
{
  Inner,
  Left,
  Right
}

public enum Order
{
  Ascending,
  Descending
}

[AttributeUsage(AttributeTargets.Property,Inherited = true)]
public sealed class DBColumnAttribute : Attribute
{
  public readonly string column;
  public readonly string? foreignTable;
  public readonly string? foreignColumn;

  public DBColumnAttribute(string column, string? foreignTable = null, string? foreignColumn = null)
  {
    if (foreignTable is null && foreignColumn is not null || foreignTable is not null && foreignColumn is null)
    {
      throw new Exception("foreignTable and foreignColumn values both need to be supported");
    }

    this.column = column;
    this.foreignTable = foreignTable;
    this.foreignColumn = foreignColumn;
  }
}

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
