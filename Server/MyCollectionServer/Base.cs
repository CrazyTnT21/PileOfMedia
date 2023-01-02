using MySqlConnector;
using System.Collections.Generic;
using System.Text;
using System.Xml.Linq;

namespace MyCollectionServer;

public class BaseT
{
  public static int count = 50;

  public static string selectLeftJoin(string table, Join[] joins, int? start, int? count)
  {
    StringBuilder? query = new ($"SELECT {Escape(table)}.*");
    if (joins.Length > 0)
      query.Append(", ");
    string[] selects = new string[joins.Length];
    for (int i = 0; i < joins.Length; i++)
    {
      selects[i] = $"{Escape(joins[i].Alias)}.{joins[i].TableColumn} AS {Escape(joins[i].Alias)}";
    }

    query.AppendJoin(',',selects);
    query.Append($" FROM {table} ");
    for (int i = 0; i < joins.Length; i++)
    {
      query.Append(joins[i].createJoin());
    }
    //select TComic.*,  
    //`Name`.`English` as Name,
    //`Description`.`English` as Description,
    //`Synopsis`.`English` as Synopsis 
    //from TComic  
    //inner JOIN `TTranslation` AS `Name` ON `TComic`.`FKName` = Name.`PK`     and TComic.PK = 5
    //left JOIN `TTranslation` AS `Description` ON `TComic`.`FKDescription` = Description.`PK` 
    //LEFT JOIN `TTranslation` AS `Synopsis` ON `TComic`.`FKSynopsis` = Synopsis.`PK`  LIMIT 0,50 ;
    query.Append($"{Limit(start, count)};");
    return query.ToString();
  }
  public static string LeftJoin(string tableName, string alias, string match, string tomatch)
  {
    return $" LEFT JOIN {Escape(tableName)} AS {Escape(alias)} ON {match} = {alias}.{Escape(tomatch)} ";
  }
  public static string Escape(string value, bool valueType = false)
  {
    if (valueType)
      return $"'{value}'";
    return $"`{value}`";
  }
  public static string Limit(int? start, int? count, bool Override = false, bool multiply = true)
  {
    start = (start is not null && start > 0) ? start : 0;
    if (!Override)
      count = (count is not null && count > 0 && count < BaseT.count) ? count : BaseT.count;

    return $" LIMIT {(multiply ? start * count : start)},{count} ";
  }
}

