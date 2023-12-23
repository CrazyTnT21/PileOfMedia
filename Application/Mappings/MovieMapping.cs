using Application.DBMapping;
using Domain.Classes;
using Domain.ValueObjects;

namespace Application.Mappings;

public sealed class MovieMapping : IMapping
{
  public IDBMapping Mapping { get; } =
    new DBMapping<Movie>()
      .Column(nameof(Movie.Id), "Id")
      .Column(nameof(Movie.Score), "Score")
      .Column(nameof(Movie.Added), "Added")
      .Column(nameof(Movie.Favorites), "Favorites")
      .Column(nameof(Movie.Members), "Members")
      .Column(nameof(Movie.Popularity), "Popularity")
      .Column(nameof(Movie.Airing), "Airing")
      .Column(nameof(Movie.Rank), "Rank")
      .Join(nameof(Movie.Cover), new Join("Image", "Movie", new Condition[]
      {
        new(new TableColumn("Image", "Id"), new TableColumn("Movie", "FKCover"))
      }))
      .Join(new Join("MovieTranslation", "Translation", "Movie", new Condition[]
      {
        new(new TableColumn("Translation", "FKTranslation"), new TableColumn("Movie", "Id")),
        new(new TableColumn("Translation", "Language"), "EN", "Language")
      }))
      .JoinProjection(nameof(Movie.Title), "Translation", "Title")
      .JoinProjection(nameof(Movie.Description), "Translation", "Description")
      .ArrayJoin(nameof(Movie.Genres))
      .ArrayJoin(nameof(Movie.Themes));
}
