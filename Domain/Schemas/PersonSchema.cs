namespace Domain.Schemas;

public static class PersonSchema
{
  public static readonly string[] excludeGet = new string[] { FKDescription,Description };
  public static readonly string[] excludeInsert = new string[] { Description };

  public const string Table = "Person";
  public const string Id = "PK";
  public const string Name = "Name";
  public const string FirstName = "FirstName";
  public const string LastName = "LastName";
  public const string Description = "Description";
  public const string FKDescription = "FKDescription";
  public const string Birthday = "Birthday";
  public const string Height = "Height";
  public const string ImageSource = "ImageSource";
}
