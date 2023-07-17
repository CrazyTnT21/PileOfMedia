using Application.Controller;
using Domain;
using Domain.Classes;
using Domain.Enums;
using Microsoft.AspNetCore.Mvc;
using MyCollectionServer.Controller.Base;
using MyCollectionServer.Core;
using MySqlConnector;
using HTTPException = Domain.Exceptions.HTTPException;

namespace MyCollectionServer.Controller;

[ApiController]
[Route("Api/Comic")]
public class ComicController : BaseAPIController<Comic>, IComicService
{
  public ComicController(ILogger logger, MySqlConnection connection) : base(logger, connection)
  {
  }

  public override async Task<Comic?> GetSingle(uint id)
  {
    var results = await new Select<Comic>()
      .QueryDB(_connection)
      .ToList();
    if (results.Count > 0)
      return results[0];
    return null;
  }

  public override async Task<IEnumerable<Comic>> Get()
  {
    var results = await new Select<Comic>()
      .AddDynamicColumn(Key.Language, "EN")
      .QueryDB(_connection)
      .ToList();
    return results;
  }

  public override async Task<Comic> Create(Comic item)
  {
    var valid = Validate(item);
    if (valid is not null)
      throw new HTTPException(valid.Value.StatusCode, valid.Value.Reason);

    object fkName = await Insert("Translation", new[] { "EN" }, new[] { "TestName" });
    object fkDescription = await Insert("Translation", new[] { "EN" }, new[] { "TestDescription" });
    string[] columns = new[] { "FKName", "FKDescription" };
    object[] values = new object[] { fkName, fkDescription };
    var result = await Insert("Comic", columns, values);

    return item;
  }

  [NonAction]
  public async Task<long> Insert(string table, string[] columns, object?[] values)
  {
    if (columns.Length != values.Length)
      throw new Exception("Columns length doesn't match values length");

    await using MySqlCommand cmd =
      new($"INSERT INTO {table}({string.Join(',', columns)}) VALUES({BaseT.RepeatUnique("@v", values.Length)})",
        _connection);

    BaseT.AddMultipleValues(cmd, BaseT.RepeatUnique("v", values.Length).Split(','), values);
    return await QueryBase.QueryDBResult(cmd);
  }

  public override async Task<Comic> Update(Comic item)
  {
    var valid = Validate(item, true);
    if (valid is not null)
      throw new HTTPException(valid.Value.StatusCode, valid.Value.Reason);

    return item;
  }

  public override HTTPError? Validate(Comic item, bool update = false)
  {
    if (item.LanguageFields is null)
      return new HTTPError(StatusCodes.Status400BadRequest, "LanguageFields is missing!");

    if (!Array.Exists(item.LanguageFields, x => x.Column.Equals("Name", StringComparison.OrdinalIgnoreCase)))
      return new HTTPError(StatusCodes.Status400BadRequest, "'Name' is required!");

    for (int i = 0; i < item.LanguageFields?.Length; i++)
      if (item.LanguageFields[i].Column.Equals("Name"))
        foreach (var val in item.LanguageFields[i].Values)
          if (!TranslationClass.LanguageExists(val.Key))
            return new HTTPError(StatusCodes.Status400BadRequest, $"Language code '{val.Key}' does not exist!");
    return null;
  }

  [HttpGet("{id}")]
  public async Task<ActionResult<Comic?>> GetSingleResult(uint id) => await GetSingle(id);

  [HttpGet]
  public async Task<ActionResult<IEnumerable<Comic>>> GetResult() => (await Get()).ToList();

  [HttpPost]
  public async Task<ActionResult<Comic>> CreateResult(Comic item) => await Create(item);

  [HttpPut]
  public async Task<ActionResult<Comic>> UpdateResult(Comic item)
  {
    return await Update(item);
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
}
