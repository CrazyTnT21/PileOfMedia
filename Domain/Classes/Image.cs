using Domain.Interfaces;

namespace Domain.Classes;

public sealed class Image : IEntity
{
  public int Id { get; set; }
  public string Uri { get; set; } = null!;
  public short Width { get; set; }
  public short Height { get; set; }
  public ImageExtension? Extension { get; set; }
}

public sealed class CreateImage
{
  public int Id { get; set; }
  public string? Uri { get; set; } = null!;
  public short? Width { get; set; }
  public short? Height { get; set; }
  public ImageExtension? Extension { get; set; }
}

public enum ImageExtension
{
  JPEG,
  JPG,
  PNG,
  GIF
}
