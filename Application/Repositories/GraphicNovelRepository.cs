using Application.DBMapping;
using Domain.Classes;
using Domain.Classes.DB;
using Domain.Common;
using Domain.Enums;
using Npgsql;

namespace Application.Repositories;

public sealed class GraphicNovelRepository : Repository, IGraphicNovelRepository
{
  public GraphicNovelRepository(NpgsqlConnection connection) : base(connection)
  {
  }

  public async Task<GraphicNovel?> GetById(uint id, Language language)
  {
    var result = await new Select<GraphicNovel>()
      .Where("Id", id)
      .OverrideValue(language, "Language")
      .UniqueResult(_connection);

    return result;
  }

  public async Task<IEnumerable<GraphicNovel>> Get(Language language)
  {
    var results = await new Select<GraphicNovel>()
      .OverrideValue(language, "Language")
      .List(_connection);
    return results;
  }

  public async Task<GraphicNovel> Create(CreateGraphicNovel item)
  {
    GraphicNovelDB GraphicNovelDb = new GraphicNovelDB
    {
      Chapters = item.Chapters,
      PublishEnd = item.PublishEnd,
      PublishStart = item.PublishStart,
      FKCover = item.Cover!.Id,
      Status = item.Status!.Value,
    };
    var result = await GetById((uint)GraphicNovelDb.Id, LanguageParser.DEFAULTLANGUAGE);
    return result!;
  }

  public async Task<GraphicNovel> Update(CreateGraphicNovel item)
  {
    throw new NotImplementedException();
  }

  public async Task Delete(uint id)
  {
    throw new NotImplementedException();
  }
}
