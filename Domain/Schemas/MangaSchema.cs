namespace Domain.Schemas;

public static class MangaSchema
{
  public const string Table = "Manga";
  public static readonly string[] excludeGet = new string[] { FKName, FKDescription, FKStatus };
  public static readonly string[] excludeInsert = new string[] { Name, Description, Status };
  public const string Id = "PK";
  public const string Name = "Name";
  public const string FKName = "FKName";
  public const string Description = "Description";
  public const string FKDescription = "FKDescription";
  public const string Status = "Status";
  public const string FKStatus = "FKStatus";
  public const string Chapters = "Chapters";
  public const string Volumes = "Volumes";
  public const string PublishStart = "PublishStart";
  public const string PublishEnd = "PublishEnd";
  public const string ImageSource = "ImageSource";
  public const string AverageScore = "AverageScore";
}
