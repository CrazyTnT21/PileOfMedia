using Application.DBMapping;
using Domain.Classes;
using Domain.Classes.DB;
using Domain.Common;
using Domain.Enums;
using Npgsql;

namespace Application.Repositories;

public sealed class GameRepository : Repository, IGameRepository
{
  public GameRepository(NpgsqlConnection connection) : base(connection)
  {
  }

  public async Task<Game?> GetById(uint id, Language language)
  {
    var result = await new Select<Game>()
      .OverrideValue(language, "Language")
      .Where("Id", id)
      .UniqueResult(_connection);
    return result;
  }

  public async Task<IEnumerable<Game>> Get(Language language)
  {
    List<Game> results = await new Select<Game>()
      .OverrideValue(language, "Language")
      .List(_connection);

    return results;
  }

  public async Task<Game> Create(CreateGame item)
  {
    GameDB GameDb = new GameDB
    {
      Published = item.Published,
      FKCover = item.Cover!.Id
    };
    var result = await GetById((uint)GameDb.Id, LanguageParser.DEFAULTLANGUAGE);
    return result!;
  }

  public async Task<Game> Update(CreateGame item)
  {
    throw new NotImplementedException();
  }

  public async Task Delete(uint id)
  {
    throw new NotImplementedException();
  }
}
