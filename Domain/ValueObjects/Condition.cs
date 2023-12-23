using System.ComponentModel;

namespace Domain.ValueObjects;

public struct Condition
{
  public TableColumn TableColumn { get; set; }
  public TableColumn? OtherColumn { get; set; }
  public object? Value { get; set; }
  public string? OverrideId { get; set; }
  public Comparison Operator { get; }
  public Stacking Stacking { get; }

  private Condition(TableColumn column,
                    Comparison compareOperator = Comparison.Equals,
                    Stacking stacking = Stacking.And)
  {
    TableColumn = column;
    Operator = compareOperator;
    Stacking = stacking;
  }

  public Condition(TableColumn column,
                   TableColumn otherColumn,
                   Comparison compareOperator = Comparison.Equals,
                   Stacking stacking = Stacking.And) : this(column, compareOperator, stacking)
  {
    OtherColumn = otherColumn;
  }

  public Condition(TableColumn column,
                   object? value,
                   string? overrideId = null,
                   Comparison compareOperator = Comparison.Equals,
                   Stacking stacking = Stacking.And) : this(column, compareOperator, stacking)
  {
    Value = value;
    OverrideId = overrideId;
  }

  public string ComparisonSign() => ComparisonSign(Operator);

  public static string ComparisonSign(Comparison comparison)
  {
    return comparison switch
    {
      Comparison.Equals => "=",
      Comparison.NotEquals => "!=",
      Comparison.LessEqual => "<=",
      Comparison.Less => "<",
      Comparison.MoreEqual => ">=",
      Comparison.More => ">",
      Comparison.Like => "LIKE",
      Comparison.NotLike => "NOT LIKE",
      Comparison.In => "IN",
      Comparison.NotIn => "NOT IN",
      _ => throw new InvalidEnumArgumentException()
    };
  }

  public string StackingSign() => StackingSign(Stacking);

  public static string StackingSign(Stacking stacking)
  {
    return stacking switch
    {
      Stacking.And => "AND",
      Stacking.Or => "OR",
      _ => throw new InvalidEnumArgumentException()
    };
  }
}

public enum Comparison
{
  Equals,
  NotEquals,
  More,
  MoreEqual,
  Less,
  LessEqual,
  Like,
  NotLike,
  ILike,
  NotILike,
  In,
  NotIn
}

public enum Stacking
{
  And,
  Or
}
