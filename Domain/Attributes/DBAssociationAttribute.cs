namespace Domain.Attributes;

[AttributeUsage(AttributeTargets.Property)]
public sealed class DBAssociationAttribute : Attribute
{
  public readonly Key Key;

  public DBAssociationAttribute(Key key)
  {
    Key = key;
  }
}
