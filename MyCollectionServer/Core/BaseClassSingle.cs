using System;
using System.Net;
using System.Reflection;
using System.Threading.Tasks;
using Microsoft.Extensions.Logging;
using Microsoft.AspNetCore.Http;
using Microsoft.AspNetCore.Mvc;
using MySqlConnector;

namespace MyCollectionServer.Core;

[ApiController]
[Route("[controller]")]
public abstract class BaseClassSingle<T> : Base<T> where T : new()
{
  public BaseClassSingle(ILogger logger, MySqlConnection mysqlCon) : base(logger, mysqlCon)
  {
  }

  [HttpGet("{id}")]
  public virtual async Task<ActionResult<T?>> Get(uint id, string? language)
  {
    return await GetItem(id, TranslationClass.GetLanguage(language));
  }

  [HttpPost]
  public virtual async Task<ActionResult<long>> Create([FromBody] T item)
  {
    MySqlTransaction trans = await _mysqlCon.BeginTransactionAsync();
    try
    {
      Validate(item);
      long Id = await CreateItem(item);
      await trans.CommitAsync();
      return Id;
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
  }

  [HttpPut]
  public virtual async Task<IActionResult> Update([FromBody] T item)
  {
    // MySqlTransaction trans = await Server.con.BeginTransactionAsync();
    try
    {
      Validate(item, true);
      await UpdateItem(item);
      //  await trans.CommitAsync();
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

    return new OkResult();
  }

  [HttpDelete]
  public virtual async Task<IActionResult> Delete(uint id)
  {
    // MySqlTransaction trans = await Server.con.BeginTransactionAsync();
    try
    {
      await DeleteItem(id);
      //  await trans.CommitAsync();
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

    return new OkResult();
  }

  [NonAction]
  public abstract Task<T?> GetItem(uint id, string language);

  [NonAction]
  public abstract Task<long> CreateItem(T item);

  [NonAction]
  public abstract Task UpdateItem(T item);

  [NonAction]
  public abstract Task DeleteItem(uint id);

}
