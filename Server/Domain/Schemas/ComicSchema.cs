namespace Domain.Schemas;

public static class ComicSchema
{
  public const string Table = "Comic";
  public const string Id = "Id";
  public const string FKName = "FKName";
  public const string FKDescription = "FKDescription";
  public const string FKStatus = "FKStatus";
  public const string Chapters = "Chapters";
  public const string Volumes = "Volumes";
  public const string PublishStart = "PublishStart";
  public const string PublishEnd = "PublishEnd";
  public const string ImageSource = "ImageSource";
  public const string AverageScore = "AverageScore";
  public static readonly string[] ExcludeGet = { FKName, FKDescription, FKStatus, "Name", "Description", "Status" };
  public static readonly string[] ExcludeInsert = { "Name", "Description", "Status" };
}
