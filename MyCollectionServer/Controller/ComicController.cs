using Application.Controller;
using Domain;
using Domain.Classes;
using Domain.Enums;
using Domain.Exceptions;
using Microsoft.AspNetCore.Mvc;
using MyCollectionServer.Controller.Base;
using MyCollectionServer.Miscellaneous;
using MySqlConnector;

namespace MyCollectionServer.Controller;

[ApiController]
[Route("Api/Comic")]
public class ComicController : APIController<Comic, ComicValidateOption>, IComicService
{
  public ComicController(ILogger logger, MySqlConnection connection) : base(logger, connection)
  {
  }

  [HttpGet]
  public async Task<ActionResult<IEnumerable<Comic>>> GetResult(string? language)
  {
    var options = new ComicGetOption(language ?? "EN");
    IEnumerable<Comic> result = await Get(options);
    return result.ToList();
  }

  [NonAction]
  public async Task<IEnumerable<Comic>> Get(ComicGetOption options)
  {
    await _connection.OpenAsync();
    var results = await new Select<Comic>()
      .AddDynamicColumn(Key.Language, options.Language)
      .QueryDB(_connection)
      .ToList();
    await _connection.CloseAsync();

    return results;
  }

  [HttpGet("{id}")]
  public async Task<ActionResult<Comic?>> GetSingleResult(uint id, string? language)
  {
    var options = new ComicGetOption(language ?? "EN");
    Comic? result = await GetSingle(id, options);
    if (result is null)
      return new NotFoundResult();
    return result;
  }

  [NonAction]
  public async Task<Comic?> GetSingle(uint id, ComicGetOption options)
  {
    await _connection.OpenAsync();

    var results = await new Select<Comic>()
      .Where("C.id", id)
      .AddDynamicColumn(Key.Language, options.Language)
      .Take(1)
      .QueryDB(_connection)
      .ToList();

    await _connection.CloseAsync();

    if (results.Count > 0)
      return results[0];

    return null;
  }

  [HttpPost]
  public async Task<ActionResult<Comic>> CreateResult(Comic item)
  {
    return await Create(item, new ComicCreateOption());
  }

  [NonAction]
  public async Task<Comic> Create(Comic item, ComicCreateOption options)
  {
    throw new NotImplementedException();
  }

  [HttpPut]
  public async Task<ActionResult<Comic>> UpdateResult(Comic item)
  {
    return await Update(item, new ComicUpdateOption());
  }

  [NonAction]
  public async Task<Comic> Update(Comic item, ComicUpdateOption options)
  {
    var validateOptions = new ComicValidateOption() { Update = true };
    var valid = Validate(item, validateOptions);
    if (valid is not null)
      throw new HTTPException(valid.Value.StatusCode, valid.Value.Reason);

    throw new NotImplementedException();
  }

  [NonAction]
  public async Task Delete(uint id)
  {
    throw new NotImplementedException();
  }

  [NonAction]
  public async Task Delete(uint[] ids)
  {
    throw new NotImplementedException();
  }

  [HttpDelete("{id}")]
  public async Task<IActionResult> DeleteResult(uint id)
  {
    await Delete(id);
    return new OkResult();
  }

  [HttpDelete]
  public async Task<IActionResult> DeleteResult(uint[] ids)
  {
    await Delete(ids);
    return new OkResult();
  }

  [NonAction]
  public override HTTPError? Validate(Comic item, ComicValidateOption options)
  {
    if (options.Update)
    {
      for (int i = 0; i < options.LanguageFields?.Length; i++)
      {
      }

      return null;
    }

    if (options.LanguageFields is null)
      return new HTTPError(StatusCodes.Status400BadRequest, "LanguageFields is missing!");

    for (int i = 0; i < options.LanguageFields.Length; i++)
    {
      // if (options.LanguageFields[i].Column.Equals("Name"))
      //   foreach (var val in item.LanguageFields[i].Values)
      //     if (!Translations.LanguageExists(val.Key))
      //       return new HTTPError(StatusCodes.Status400BadRequest, $"Language code '{val.Key}' does not exist!");
    }

    if (!Array.Exists(options.LanguageFields, x => x.Column.Equals("Name", StringComparison.OrdinalIgnoreCase)))
      return new HTTPError(StatusCodes.Status400BadRequest, "'Name' is required!");

    return null;
  }
}

public struct ComicValidateOption
{
  public bool Update;
  public LanguageField[]? LanguageFields;
}
