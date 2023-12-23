namespace Domain.ValueObjects;

public struct JoinMapping
{
  public uint? Property;
  public Join Join;

  public JoinMapping(uint? property, Join join)
  {
    Property = property;
    Join = join;
  }
}
