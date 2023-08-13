using Domain.Enums;

namespace MyCollectionServer.Miscellaneous;

public sealed class TableColumn
{
  public readonly Dictionary<string, List<ColumnMapping>> Columns = new();

  public readonly Dictionary<string, (Key, Join, uint[])> RuntimeJoin = new();

  public readonly Dictionary<string, Join> Joins = new();
}
