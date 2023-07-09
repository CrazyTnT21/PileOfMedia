namespace Domain.Schemas;

public static class GameSchema
{
  public static readonly string[] ExcludeGet = new string[] { FKName, FKDescription,FKStatus };
  public static readonly string[] ExcludeInsert = new string[] { Name, Description,Status };

  public const string Table = "Game";
  public const string Id = "PK";
  public const string Name = "Name";
  public const string FKName = "FKName";
  public const string Description = "Description";
  public const string FKDescription = "FKDescription";
  public const string Published = "Published";
  public const string AverageScore = "AverageScore";
  public const string FKStatus = "FKStatus";
  public const string Status = "Status";
  public const string ImageSource = "ImageSource";
}
