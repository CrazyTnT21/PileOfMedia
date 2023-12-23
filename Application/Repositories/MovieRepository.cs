using Application.DBMapping;
using Domain.Classes;
using Domain.Classes.DB;
using Domain.Common;
using Domain.Enums;
using Domain.Repositories;
using Domain.ValueObjects;
using Npgsql;

namespace Application.Repositories;

public sealed class MovieRepository : Repository, IMovieRepository
{
  public MovieRepository(NpgsqlConnection connection) : base(connection)
  {
  }

  public async Task<Movie?> GetById(uint id, Language language)
  {
    var result = await new Select<Movie>()
      .OverrideValue(language, "Language")
      .Where("Id", id)
      .UniqueResult(_connection);

    return result;
  }

  public async Task<IEnumerable<Movie>> Get(Language language)
  {
    List<Movie> results = await new Select<Movie>()
      .OverrideValue(language, "Language")
      .Join("Title",new Join("MovieTranslation", "Movie", new Condition[]
      {
        new(new TableColumn("MovieTranslation", "FKTranslation"), new TableColumn("Movie", "Id")),
        new(new TableColumn("MovieTranslation", "Language"), "EN"),
        new(new TableColumn("Translation", "Title"), null),
      })
        )
      .List(_connection);
    return results;
  }

  public async Task<IEnumerable<User>> GetUsersByMovie(uint id)
  {
    List<User> results = await new Select<User>("UserXMovie")
      .Join(new Join("UserXMovie", "User",
                     new Condition(new TableColumn("UserXMovie", "FKUser"), new TableColumn("User", "Id"))))
      .List(_connection);
    return results;
  }

  public async Task<IEnumerable<uint>> GetUserIdsByMovie(uint id)
  {
    List<uint> results = await new Select<uint>(typeof(Movie))
      .Projection("Id")
      .List(_connection);
    return results;
  }

  public async Task<Movie> Create(CreateMovie item)
  {
    MovieDB MovieDb = new MovieDB
    {
      Airing = item.Airing,
      Length = item.Length,
      FKCover = item.Cover.Id
    };
    object id = Insert(MovieDb, "Movie");

    var result = await GetById((uint)id, LanguageParser.DEFAULTLANGUAGE);
    return result!;
  }


  public async Task<Movie> Update(CreateMovie item)
  {
    throw new NotImplementedException();
  }

  public async Task Delete(uint id)
  {
    //Delete From Translation
    //Delete From MovieXCharacter
    //Delete From MovieXGenre
    //Delete From MovieXTheme
    //Delete From MovieXCreator
    //Delete From UserXMovie
    //Delete
    throw new NotImplementedException();
  }
}
