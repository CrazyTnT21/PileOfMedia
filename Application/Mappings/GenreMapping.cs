using Application.DBMapping;
using Domain.Classes;
using Domain.ValueObjects;

namespace Application.Mappings;

public sealed class GenreMapping : IMapping
{
  public IDBMapping Mapping { get; } =
    new DBMapping<Genre>()
      .Column(nameof(Genre.Id), "Id")
      .Join(new Join("GenreTranslation", "Translation", "Genre",
                     new Condition(new TableColumn("GenreTranslation", "FKTranslation"),
                                   new TableColumn("Genre", "Id")),
                     new Condition(new TableColumn("GenreTransLation", "Language"), "EN")))
      .JoinProjection(nameof(Genre.Name), "Translation", "Name");
}
