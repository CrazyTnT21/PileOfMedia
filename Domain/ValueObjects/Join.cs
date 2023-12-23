using System.ComponentModel;
using System.Text;
using Domain.Common;
using Domain.Enums;

namespace Domain.ValueObjects;

public readonly struct Join
{
  public readonly string Table;
  public readonly bool TableEscape;
  public readonly string? Alias;
  public readonly string OnTable;
  public readonly Condition[] Conditions;
  public readonly JoinType JoinType;

  public Join(string table, bool tableEscape, string? alias, string onTable, params Condition[] conditions)
  {
    if (conditions.Length == 0)
      throw new Exception("Joining with no conditions is not possible");

    Table = table;
    TableEscape = tableEscape;
    Alias = alias;
    OnTable = onTable;
    Conditions = conditions;
  }

  public Join(string table,
              string alias,
              string onTable,
              params Condition[] conditions) : this(table, false, alias, onTable, conditions)
  {
  }

  public Join(string table,
              bool tableEscape,
              string onTable,
              params Condition[] conditions) : this(table, tableEscape, null, onTable, conditions)
  {
  }

  public Join(string table,
              string onTable,
              params Condition[] conditions) : this(table, false, null, onTable, conditions)
  {
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

  private void AppendCondition(StringBuilder builder, Condition condition)
  {
    builder.Append(condition.TableColumn.Combined);
    builder.Append(condition.ComparisonSign());
    if (condition.OtherColumn is null)
    {
      builder.Append('\'');
      builder.Append(condition.Value);
      builder.Append('\'');
    }
    else
    {
      builder.Append(condition.OtherColumn.Value.Combined);
    }
  }

  public string CreateJoin(JoinType? joinType = null)
  {
    StringBuilder builder = joinType is null
      ? new StringBuilder("JOIN ")
      : new StringBuilder(JoinString(joinType.Value) + " JOIN ");

    if (TableEscape)
    {
      builder.Append('"');
      builder.Append(Table);
      builder.Append('"');
    }
    else
      builder.Append(Table);

    if (Alias is not null)
    {
      builder.Append(" AS ");
      builder.Append(Alias);
    }

    builder.Append(" ON ");
    AppendCondition(builder, Conditions[0]);
    for (int i = 1; i < Conditions.Length; i++)
    {
      builder.Append(' ');
      builder.Append(Conditions[i].StackingSign());
      builder.Append(' ');
      AppendCondition(builder, Conditions[i]);
    }

    return builder.ToString();
  }
}

public readonly struct TableColumn
{
  public readonly string Table;
  public readonly string Column;

  public readonly string Combined;

  public TableColumn(string table, string column)
  {
    Table = table;
    Column = column;
    Combined = table + "." + column;
  }
}
