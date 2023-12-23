using Application.DBMapping;
using Domain.Classes;
using Domain.ValueObjects;

namespace Application.Mappings;

public sealed class GameMapping : IMapping
{
  public IDBMapping Mapping { get; } =
    new DBMapping<Game>()
      .Column(nameof(Game.Id), "Id")
      .Column(nameof(Game.Published), "Published")
      .Column(nameof(Game.Score), "Score")
      .Column(nameof(Game.Added), "Added")
      .Join(nameof(Game.Cover),
            new Join("Image", "Game",
                     new Condition(new TableColumn("Image", "Id"), new TableColumn("Game", "FKCover"))))
      .Join(new Join("GameTranslation","Translation", "Game",
                     new Condition(new TableColumn("GameTranslation", "FKTranslation"), new TableColumn("Game", "Id")),
                     new Condition(new TableColumn("GameTransLation", "Language"), "EN")))
      .JoinProjection(nameof(Game.Title), "Translation", "Title")
      .JoinProjection(nameof(Game.Description), "Translation", "Description");
}
