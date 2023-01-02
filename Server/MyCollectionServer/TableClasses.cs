using MyCollectionServer;
using MySqlConnector;

public class TLanguage : PKClass
{
  public uint? PK { get; init; }
  public string Language { get; init; }
  public string ColumnName { get; init; }
};
public class TPublish : PKClass
{
  public uint? PK { get; init; }
  public uint FKPublish { get; set; }
}
public class TRelation : PKClass
{
  public uint? PK { get; init; }
  public uint FKRelation { get; set; }
}
public class TGenre : PKClass
{
  public uint? PK { get; init; }
  public uint FKGenre { get; set; }
}
public class TStatus : PKClass
{
  public uint? PK { get; init; }
  public uint FKStatus { get; set; }
}
public class TUserStatus : PKClass
{
  public uint? PK { get; init; }
  public uint FKUserStatus { get; set; }
}
public class TTheme : PKClass
{
  public uint? PK { get; init; }
  public uint FKTheme { get; set; }
}
public class TRole : PKClass
{
  public uint? PK { get; init; }
  public uint FKRole { get; set; }
}

public class TUser : PKClass
{
  [DBColumn("PK")]
  public uint? PK { get; init; }
  [DBColumn("FKPerson")]
  public uint? FKPerson { get; init; }
  [DBColumn("Name")]
  public string Name { get; set; }
  [DBColumn("Joined")]
  public DateTime Joined { get; set; }
  [DBColumn("Description")]
  public string? Description { get; set; }
  [DBColumn("ImageSource")]
  public string? ImageSource { get; set; }
  [DBColumn("MangaAverage")]
  public float? MangaAverage { get; set; }
  [DBColumn("ComicAverage")]
  public float? ComicAverage { get; set; }
  [DBColumn("TVShowAverage")]
  public float? TVShowAverage { get; set; }
  [DBColumn("MovieAverage")]
  public float? MovieAverage { get; set; }
  [DBColumn("AnimeAverage")]
  public float? AnimeAverage { get; set; }
  [DBColumn("BookAverage")]
  public float? BookAverage { get; set; }
  [DBColumn("CartoonAverage")]
  public float? CartoonAverage { get; set; }
  [DBColumn("GameAverage")]
  public float? GameAverage { get; set; }
};
public class TComic : PKClass, Media
{
  [DBColumn("PK")]
  public uint? PK { get; init; }
  [DBColumn("FKName")]
  public uint? FKName { get; init; }
  [DBColumn("FKDescription")]
  public uint? FKDescription { get; init; }
  [DBColumn("FKSynopsis")]
  public uint? FKSynopsis { get; init; }
  [DBColumn("Chapters")]
  public ushort? Chapters { get; set; }
  [DBColumn("Volumes")]
  public ushort? Volumes { get; set; }
  [DBColumn("PublishStart")]
  public DateTime? PublishStart { get; set; }
  [DBColumn("PublishEnd")]
  public DateTime? PublishEnd { get; set; }
  [DBColumn("ImageSource")]
  public string? ImageSource { get; set; }
  [DBColumn("AverageScore")]
  public float? AverageScore { get; set; }
  [DBColumn("Name")]
  public string? Name { get; set; }
  [DBColumn("Description")]
  public string? Description { get; set; }
  [DBColumn("Synopsis")]
  public string? Synopsis { get; set; }

  public LanguageField[]? LanguageFields { get; set; }
};
public class TComicVolume : PKClass
{
  [DBColumn("PK")]
  public uint? PK { get; init; }
  [DBColumn("FKComic")]
  public TComic Comic { get; init; }
  [DBColumn("FKTitle")]
  public uint? FKTitle { get; init; }
  [DBColumn("FKSynopsis")]
  public uint? FKSynopsis { get; set; }
  [DBColumn("Pages")]
  public uint? Pages { get; set; }
  [DBColumn("PublishDate")]
  public DateTime? PublishDate { get; set; }
  [DBColumn("PublishEnd")]
  public DateTime? PublishEnd { get; set; }
  [DBColumn("ImageSource")]
  public string? ImageSource { get; set; }
  [DBColumn("AverageScore")]
  public float? AverageScore { get; set; }
  [DBColumn("Title")]
  public string? Title { get; set; }
  [DBColumn("Description")]
  public string? Description { get; set; }
  [DBColumn("Synopsis")]
  public string? Synopsis { get; set; }

  public LanguageField[]? LanguageFields { get; set; }
};
public class TComicChapter : PKClass
{
  [DBColumn("PK")]
  public uint? PK { get; init; }
  [DBColumn("FKComic")]
  public TComic Comic { get; init; }
  [DBColumn("FKComicVolume")]
  public TComicVolume? ComicVolume { get; init; }
  [DBColumn("FKTitle")]
  public uint? FKTitle { get; init; }
  [DBColumn("FKSynopsis")]
  public uint? FKSynopsis { get; set; }
  [DBColumn("Pages")]
  public uint? Pages { get; set; }
  [DBColumn("PublishDate")]
  public DateTime? PublishDate { get; set; }
  [DBColumn("PublishEnd")]
  public DateTime? PublishEnd { get; set; }
  [DBColumn("ImageSource")]
  public string? ImageSource { get; set; }
  [DBColumn("AverageScore")]
  public float? AverageScore { get; set; }
  [DBColumn("Title")]
  public string? Title { get; set; }
  [DBColumn("Description")]
  public string? Description { get; set; }
  [DBColumn("Synopsis")]
  public string? Synopsis { get; set; }

  public LanguageField[]? LanguageFields { get; set; }
};
//Other
public interface PKClass
{
  public uint? PK { get; init; }
}
public interface Media
{
  public uint? FKName { get; init; }
  public uint? FKDescription { get; init; }
  public uint? FKSynopsis { get; init; }

  public string? ImageSource { get; set; }
  public float? AverageScore { get; set; }

  public string? Name { get; set; }
  public string? Description { get; set; }
  public string? Synopsis { get; set; }
}
public class Translation
{
  public Translation(string value, string language)
  {
    Value = value;
    Language = language;
  }
  public string Language { get; set; }
  public string Value { get; set; }
}
public class LanguageField
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

public class Join
{
  public Join(string table,string tableColumn, string alias, string ontable, string oncolumn, string matchColumn, string? andtable = null, string? andcolumn = null, object? andvalue = null, JoinType joinType = JoinType.Left)
  {
    Table = table;
    TableColumn = tableColumn;
    Alias = alias;
    MatchColumn = matchColumn;
    AndTable = andtable;
    andColumn = andcolumn;
    andValue = andvalue;
    OnTable = ontable;
    OnColumn = oncolumn;
    JoinType = joinType;
  }
  public string Table;
  public string TableColumn;
  public string Alias;
  public string OnTable;
  public string OnColumn;
  public string MatchColumn;
  public string? AndTable;
  public string? andColumn;
  public object? andValue;
  public JoinType JoinType;

  public string createJoin()
  {
    string jointype = string.Empty;
    //inner JOIN `TTranslation` AS `Name` ON `TComic`.`FKName` = Name.`PK`     and TComic.PK = 5
    switch (JoinType)
    {
      case JoinType.Inner:
        jointype = "INNER";
        break;
      case JoinType.Left:
        jointype = "LEFT";
        break;
      case JoinType.Right:
        jointype = "RIGHT";
        break;
    }
    return $"{jointype} JOIN {BaseT.Escape(Table)} AS {BaseT.Escape(Alias)} ON {BaseT.Escape(OnTable)}.{BaseT.Escape(OnColumn)} = {BaseT.Escape(Alias)}.{BaseT.Escape(MatchColumn)} {(AndTable is not null && andColumn is not null && andValue is not null ? ($" AND {BaseT.Escape(AndTable)}.{BaseT.Escape(andColumn)} = {andValue} ") : string.Empty) }";
  }
}
public enum JoinType
{
  Inner, Left, Right
}
[AttributeUsage(AttributeTargets.Property)]
public class DBColumnAttribute : Attribute
{
  public string column;

  public DBColumnAttribute(string column)
  {
    this.column = column;
  }
}
