using Application.DBMapping;
using Domain.Classes;
using Domain.ValueObjects;

namespace Application.Mappings;

public sealed class GraphicNovelMapping : IMapping
{
  public IDBMapping Mapping { get; } =
    new DBMapping<GraphicNovel>()
      .Column(nameof(GraphicNovel.Id), "Id")
      .Column(nameof(GraphicNovel.Chapters), "Chapters")
      .Column(nameof(GraphicNovel.Score), "Score")
      .Column(nameof(GraphicNovel.Volumes), "Volumes")
      .Column(nameof(GraphicNovel.Status), "Status")
      .Column(nameof(GraphicNovel.PublishEnd), "PublishEnd")
      .Column(nameof(GraphicNovel.PublishStart), "PublishStart")
      .Column(nameof(GraphicNovel.Added), "Added")
      .Join(nameof(GraphicNovel.Cover),
            new Join("Image", "GraphicNovel",
                     new Condition(
                       new TableColumn("Image", "Id"),
                       new TableColumn("GraphicNovel", "FKCover"))))
      .Join(new Join("GraphicNovelTranslation", "Translation", "GraphicNovel",
                     new Condition(
                       new TableColumn("Translation", "FKTranslation"),
                       new TableColumn("GraphicNovel", "Id")),
                     new Condition(new TableColumn("Translation", "Language"), "EN", "Language")))
      .JoinProjection(nameof(GraphicNovel.Title), "Translation", "Title")
      .JoinProjection(nameof(GraphicNovel.Description), "Translation", "Description");
}
