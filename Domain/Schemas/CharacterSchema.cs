namespace Domain.Schemas;

public static class CharacterSchema
{
  public const string Table = "Character";
  public static readonly string[] ExcludeGet = new string[] { FKName, FKDescription,FKLastName,FKFirstName,Name, Description,LastName,FirstName };
  public static readonly string[] ExcludeInsert = new string[] { Name, Description,LastName,FirstName };
  public const string Id = "PK";
  public const string Name = "Name";
  public const string FKName = "FKName";
  public const string FirstName = "FirstName";
  public const string FKFirstName = "FKFirstName";
  public const string LastName = "Name";
  public const string FKLastName = "FKLastName";
  public const string Description = "Description";
  public const string FKDescription = "FKDescription";
  public const string ImageSource = "ImageSource";
  public const string Birthday = "Birthday";
  public const string Height = "Height";
}
