using System;
using System.Collections.Generic;
using System.Net;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Http;
using Microsoft.AspNetCore.Mvc;
using MyCollectionServer.Core;
using MySqlConnector;
using Microsoft.Extensions.Logging;
namespace MyCollectionServer.Pages;

[Route("api/manga")]
public class MangaClass : BaseClass<Manga>
{
  public MangaClass(ILogger logger, MySqlConnection mysqlCon) : base(logger, mysqlCon)
  {
  }

  public override async Task<Manga?> GetItem(uint id)
  {
    string language = "EN";
    Join[] leftjoins =
    {
    //   new Join("Translation", language, MangaSchema.name, MangaSchema.table, MangaSchema.fkname, "PK",
    //     MangaSchema.table, MangaSchema.pk, id, JoinType.Inner),
    //   new Join("Translation", language, MangaSchema.status, MangaSchema.table, MangaSchema.fkstatus, "PK",
    //     MangaSchema.table),
    //   new Join("Translation", language, MangaSchema.description, MangaSchema.table, MangaSchema.fkdescription, "PK"),
    //   new Join("MangaXCreator", "FKPerson", "Creator", MangaSchema.table, "PK", "FKManga"),
    };
    var item = await QueryBase.QueryDB<Manga>(
      new MySqlCommand(BaseT.SelectLeftJoin(MangaSchema.table, leftjoins, 0, 1), _mysqlCon),
      MangaSchema.excludeGet);
    if (item.Count != 0)
      return item[0];
    return null;
  }

  public override async Task<List<Manga>> GetItems(uint? start, uint? limit, string? orderColumn, Order? order)
  {
    Join[] leftjoins =
    {
      // new Join("Translation", language, MangaSchema.name, MangaSchema.table, MangaSchema.fkname, "PK"),
      // new Join("Translation", language, MangaSchema.status, MangaSchema.table, MangaSchema.fkstatus, "PK",
      //   MangaSchema.table),
      // new Join("Translation", language, MangaSchema.description, MangaSchema.table, MangaSchema.fkdescription, "PK"),
      // new Join("MangaXCreator", "FKPerson", "Creator", MangaSchema.table, "PK", "FKManga"),
    };
    return await QueryBase.QueryDB<Manga>(
      new MySqlCommand(BaseT.SelectLeftJoin(MangaSchema.table, leftjoins, start, limit), _mysqlCon),
      MangaSchema.excludeGet);
  }

  public override async Task<long> CreateItem(Manga item)
  {
    uint FKName =
      (uint)await new TranslationClass(_logger, _mysqlCon).CreateItem(MangaSchema.fkname, item.LanguageFields!);
    uint? FKDescription =
      (uint?)await new TranslationClass(_logger, _mysqlCon).CreateItem(MangaSchema.fkdescription, item.LanguageFields);
    List<string> columns = GetColumns(MangaSchema.excludeInsert);
    List<object?> values = GetValues(item, columns);
    values.AddRange(new object?[] { FKName, FKDescription });
    columns.AddRange(new[] { MangaSchema.fkname, MangaSchema.fkdescription });
    return await Insert(MangaSchema.table, columns.ToArray(), values.ToArray());
  }

  public override async Task UpdateItem(Manga item)
  {
    throw new NotImplementedException();
  }

  public override Task DeleteItem(uint id)
  {
    throw new NotImplementedException();
  }

  public override async Task DeleteItems(uint[] id)
  {
    throw new NotImplementedException();
  }

  public override void Validate(Manga item, bool update = false)
  {
    if (!BaseT.IsValidColumn(MangaSchema.fkname, item.LanguageFields))
      throw new HTTPException(StatusCodes.Status400BadRequest);
  }
}
