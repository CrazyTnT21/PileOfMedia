using Application.DBMapping;
using Domain.Classes;

namespace Application.Mappings;

public sealed class ImageMapping : IMapping
{
  public IDBMapping Mapping { get; } =
    new DBMapping<Image>()
      .Column(nameof(Image.Id), "Id")
      .Column(nameof(Image.Extension), "Extension")
      .Column(nameof(Image.Height), "Height")
      .Column(nameof(Image.Width), "Width")
      .Column(nameof(Image.Uri), "Uri");
}
