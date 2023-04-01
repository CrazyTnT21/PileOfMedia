namespace Domain.Schemas;

public static class ComicSchema
{
  public const string Table = "Comic";
  public static readonly string[] ExcludeGet = new string[] { FKName, FKDescription, FKStatus,"Name","Description","Status" };
  public static readonly string[] ExcludeInsert = new string[] { "Name","Description","Status"  };
  public const string Id = "PK";
  public const string FKName = "FKName";
  public const string FKDescription = "FKDescription";
  public const string FKStatus = "FKStatus";
  public const string Chapters = "Chapters";
  public const string Volumes = "Volumes";
  public const string PublishStart = "PublishStart";
  public const string PublishEnd = "PublishEnd";
  public const string ImageSource = "ImageSource";
  public const string AverageScore = "AverageScore";
}
