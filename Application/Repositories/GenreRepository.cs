using Application.DBMapping;
using Domain.Classes;
using Domain.Classes.DB;
using Domain.Common;
using Domain.Enums;
using Npgsql;

namespace Application.Repositories;

public sealed class GenreRepository : Repository, IGenreRepository
{
  public GenreRepository(NpgsqlConnection connection) : base(connection)
  {
  }

  public async Task<Genre?> GetById(uint id, Language language)
  {
    var result = await new Select<Genre>()
      .OverrideValue(language, "Language")
      .Where("Id", id)
      .UniqueResult(_connection);

    return result;
  }

  public async Task<Genre?> GetByName(string name, Language language)
  {
    var result = await new Select<Genre>()
      .OverrideValue(language, "Language")
      .Where("Name", name)
      .UniqueResult(_connection);

    return result;
  }

  public async Task<IEnumerable<Genre>> Get(Language language)
  {
    List<Genre> results = await new Select<Genre>()
      .OverrideValue(language, "Language")
      .List(_connection);
    return results;
  }

  public async Task<Genre> Create(CreateGenre item)
  {
    GenreDB GenreDb = new GenreDB();
    var result = await GetById((uint)GenreDb.Id, LanguageParser.DEFAULTLANGUAGE);
    return result!;
  }

  public async Task<Genre> Update(CreateGenre item)
  {
    throw new NotImplementedException();
  }

  public async Task Delete(uint id)
  {
    //Delete From Translation
    //Delete From GenreXCharacter
    //Delete From GenreXGenre
    //Delete From GenreXTheme
    //Delete From GenreXCreator
    //Delete From UserXGenre
    //Delete
    throw new NotImplementedException();
  }
}
