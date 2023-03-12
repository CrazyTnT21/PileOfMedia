using System.Net;
using System.Reflection;
using Microsoft.AspNetCore.Mvc;
using MyCollectionServer.Core;
using MySqlConnector;

namespace MyCollectionServer.Pages;

[Route("api/comic")]
public sealed class ComicClass : BaseClass<Comic>
{
  public ComicClass(ILogger logger, MySqlConnection mysqlCon) : base(logger, mysqlCon)
  {
  }

  public async Task<ActionResult<List<Comic>>> Get(string? language, uint? start, uint? limit, string? orderColumn,
    Order? order)
  {
    List<Comic> items;
    try
    {
      items = await GetItems(start, limit, orderColumn, order);
    }
    catch (HTTPException ex)
    {
      BaseT.LogWarning(_logger, ex);
      return new StatusCodeResult(ex.StatusCode);
    }
    catch (Exception ex)
    {
      BaseT.LogError(_logger, ex);
      return new StatusCodeResult(StatusCodes.Status500InternalServerError);
    }

    return items;
  }

  public async Task<ActionResult<Comic?>> GetSingle(string? language, uint id)
  {
    Comic? item;
    language = TranslationClass.GetLanguage(language);
    try
    {
      item = await GetItem(language, id);
    }
    catch (HTTPException ex)
    {
      BaseT.LogWarning(_logger, ex);
      return new StatusCodeResult(ex.StatusCode);
    }
    catch (Exception ex)
    {
      BaseT.LogError(_logger, ex);
      return new StatusCodeResult(StatusCodes.Status500InternalServerError);
    }

    if (item is null)
      return new NotFoundResult();
    return item;
  }

  public async Task<Comic?> GetItem(string language, uint id)
  {
    var result = await SelectItem(id, language, null, null, null, null).QueryDB<Comic>(_mysqlCon);
    if (result.Count > 1)
      throw new Exception("GetItem should not query more than 1 item");

    for (int i = 0; i < result.Count; i++)
    {
      var charSelect = new Select("ComicXCharacter")
        .Where("FKComic", result[i].PK)
        .Join(new Join(CharacterSchema.table, CharacterSchema.pk, "ComicXCharacter", "FKCharacter"));
      CharacterClass.GetX(charSelect, language);

      var creatorSelect = new Select("ComicXCreator")
        .Where("FKComic", result[i].PK)
        .Join(new Join("Role", "PK", "ComicXCreator", "FKRole"))
        .Join(new Join(PersonSchema.table, PersonSchema.pk, "ComicXCreator", "FKPerson"));
      CreatorClass.GetX(creatorSelect, language);

      result[i].characters = (await charSelect.QueryDB<Character>(_mysqlCon)).ToArray();
      result[i].creators = (await creatorSelect.QueryDB<Creator>(_mysqlCon)).ToArray();
    }

    if (result.Count != 0)
      return result[0];
    return null;
  }

  public override Task<Comic?> GetItem(uint id) => GetItem("EN", id);

  public override Task<List<Comic>> GetItems(uint? start, uint? limit, string? orderColumn, Order? order) =>
    GetItems("EN", start, limit, orderColumn, order);

  public Select SelectItem(uint? id, string language, uint? start, uint? limit, string? orderColumn, Order? order)
  {
    var select = new Select("Comic")
      .AddColumns(ComicSchema.table, GetColumns(ComicSchema.excludeGet))
      .AddColumn(language, "TStatus", "Status")
      .AddColumn(language, "TName", "Name")
      .AddColumn(language, "TDescription", "Description")
      .Join(new Join(TranslationSchema.table, TranslationSchema.pk, ComicSchema.table, ComicSchema.fkname, "TName"))
      .Join(new Join(TranslationSchema.table, TranslationSchema.pk, ComicSchema.table, ComicSchema.fkdescription,
        "TDescription"))
      .Join(new Join("Status", "PK", ComicSchema.table, ComicSchema.fkstatus))
      .Join(new Join(TranslationSchema.table, TranslationSchema.pk, "Status", "FKStatus", "TStatus"));
    if (id is not null)
      select.Where("PK", id);
    else if (limit is not null)
      select.Limit((uint)limit);
    return select;
  }

  public async Task<List<Comic>> GetItems(string language, uint? start, uint? limit, string? orderColumn, Order? order)
  {
    var result = await SelectItem(null, language, start, limit, orderColumn, order).QueryDB<Comic>(_mysqlCon);

    for (int i = 0; i < result.Count; i++)
    {
      var charSelect = new Select("ComicXCharacter")
        .Where("FKComic", result[i].PK)
        .Join(new Join(CharacterSchema.table, CharacterSchema.pk, "ComicXCharacter", "FKCharacter"));
      charSelect = CharacterClass.GetX(charSelect, language);

      var creatorSelect = new Select("ComicXCreator")
        .Where("FKComic", result[i].PK)
        .Join(new Join("Role", "PK", "ComicXCreator", "FKRole"))
        .Join(new Join(PersonSchema.table, PersonSchema.pk, "ComicXCreator", "FKPerson"));
      creatorSelect = CreatorClass.GetX(creatorSelect, language);

      result[i].characters = (await charSelect.QueryDB<Character>(_mysqlCon)).ToArray();
      result[i].creators = (await creatorSelect.QueryDB<Creator>(_mysqlCon)).ToArray();
    }

    return result;
  }

  public override async Task UpdateItem(Comic item)
  {
    throw new NotImplementedException();
  }

  public override async Task DeleteItem(uint id)
  {
    throw new NotImplementedException();
  }

  public override Task DeleteItems(uint[] id)
  {
    throw new NotImplementedException();
  }

  public override async Task<long> CreateItem(Comic item)
  {
    Validate(item);
    var trans = new TranslationClass(_logger, _mysqlCon);
    uint FKName = (uint)await trans.CreateItem(ComicSchema.fkname, item.LanguageFields);
    uint? FKDescription = (uint?)await trans.CreateItem(ComicSchema.fkdescription, item.LanguageFields);
    List<string> columns = GetColumns(ComicSchema.excludeInsert);
    List<object?> values = GetValues(item, columns);
    values.AddRange(new object?[] { FKName, FKDescription });
    columns.AddRange(new[] { ComicSchema.fkname, ComicSchema.fkdescription });
    return await Insert(ComicSchema.table, columns.ToArray(), values.ToArray());
  }

  public override void Validate(Comic item, bool update = false)
  {
    if (!BaseT.IsValidColumn(ComicSchema.fkname, item.LanguageFields))
      throw new HTTPException(StatusCodes.Status400BadRequest, "'Name' is not valid!");
  }
}
