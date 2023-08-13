using System.ComponentModel;

namespace MyCollectionServer.Miscellaneous;

public struct Join
{
  public readonly string Table;
  public readonly string? Alias;
  public readonly string Column;
  public readonly string JoinTable;
  public readonly string JoinColumn;

  public Join(string table, string column, string joinTable, string joinColumn, string? alias = null)
  {
    Table = table;
    Alias = alias;
    Column = column;
    JoinTable = joinTable;
    JoinColumn = joinColumn;
  }

  public string CreateJoin(JoinType? joinType = null)
  {
    if (joinType is null)
    {
      if (Alias is null)
      {
        return $" JOIN `{Table}` ON `{Table}`.`{Column}` = `{JoinTable}`.`{JoinColumn}` ";
      }

      return $" JOIN `{Table}` AS `{Alias}` ON `{Alias}`.`{Column}` = `{JoinTable}`.`{JoinColumn}` ";
    }

    if (Alias is null)
    {
      return $" {JoinString(joinType.Value)} JOIN `{Table}` ON `{Table}`.`{Column}` = `{JoinTable}`.`{JoinColumn}` ";
    }

    return
      $" {JoinString(joinType.Value)} JOIN `{Table}` AS `{Alias}` ON `{Alias}`.`{Column}` = `{JoinTable}`.`{JoinColumn}` ";
  }

  public static string JoinString(JoinType type)
  {
    switch (type)
    {
      case JoinType.Inner:
        return "INNER";
      case JoinType.Left:
        return "LEFT";
      case JoinType.Right:
        return "RIGHT";
      case JoinType.Cross:
        return "CROSS";
      default: throw new InvalidEnumArgumentException();
    }
  }
}
