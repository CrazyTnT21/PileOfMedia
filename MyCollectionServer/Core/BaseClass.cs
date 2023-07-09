using System;
using System.Collections.Generic;
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
public abstract class BaseClass<T> : Base<T>, ICrud<T> where T : new()
{
  public BaseClass(ILogger logger, MySqlConnection mysqlCon) : base(logger, mysqlCon)
  {
  }

  [HttpGet("{id}")]
  public virtual async Task<ActionResult<T?>> GetSingle(uint id)
  {
    T? item;
    try
    {
      item = await GetItem(id);
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

  [HttpGet]
  public virtual async Task<ActionResult<List<T>>> Get(uint? start, uint? limit, string? orderColumn, Order? order)
  {
    List<T> items;
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

  [HttpPost]
  public virtual async Task<ActionResult<long>> Create([FromBody] T item)
  {
    // MySqlTransaction trans = await _mysqlCon.BeginTransactionAsync();
    try
    {
      Validate(item);
      long Id = await CreateItem(item);
      // await trans.CommitAsync();
      return Id;
    }
    catch (HTTPException ex)
    {
      // await trans.RollbackAsync();
      BaseT.LogWarning(_logger, ex);
      return new StatusCodeResult(ex.StatusCode);
    }
    catch (Exception ex)
    {
      // await trans.RollbackAsync();
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
      // await trans.RollbackAsync();
      BaseT.LogWarning(_logger, ex);
      return new StatusCodeResult(ex.StatusCode);
    }
    catch (Exception ex)
    {
      // await trans.RollbackAsync();
      BaseT.LogError(_logger, ex);
      return new StatusCodeResult(StatusCodes.Status500InternalServerError);
    }

    return new OkResult();
  }

  [HttpDelete]
  public virtual async Task<IActionResult> Delete(uint id)
  {
    // MySqlTransaction trans = await _mysqlCon.BeginTransactionAsync();
    try
    {
      await DeleteItem(id);
      //  await trans.CommitAsync();
    }
    catch (HTTPException ex)
    {
      // await trans.RollbackAsync();
      BaseT.LogWarning(_logger, ex);
      return new StatusCodeResult(ex.StatusCode);
    }
    catch (Exception ex)
    {
      // await trans.RollbackAsync();
      BaseT.LogError(_logger, ex);
      return new StatusCodeResult(StatusCodes.Status500InternalServerError);
    }

    return new OkResult();
  }

  [HttpDelete]
  public virtual async Task<IActionResult> Delete(uint[] ids)
  {
    // MySqlTransaction trans = await _mysqlCon.BeginTransactionAsync();
    try
    {
      await DeleteItems(ids);
      //  await trans.CommitAsync();
    }
    catch (HTTPException ex)
    {
      // await trans.RollbackAsync();
      BaseT.LogWarning(_logger, ex);
      return new StatusCodeResult(ex.StatusCode);
    }
    catch (Exception ex)
    {
      // await trans.RollbackAsync();
      BaseT.LogError(_logger, ex);
      return new StatusCodeResult(StatusCodes.Status500InternalServerError);
    }

    return new OkResult();
  }

  [NonAction]
  public abstract Task<T?> GetItem(uint id);

  [NonAction]
  public abstract Task<List<T>> GetItems(uint? start, uint? limit, string? orderColumn, Order? order);

  [NonAction]
  public abstract Task<long> CreateItem(T item);

  [NonAction]
  public abstract Task UpdateItem(T item);

  [NonAction]
  public abstract Task DeleteItem(uint id);

  [NonAction]
  public abstract Task DeleteItems(uint[] id);
}
