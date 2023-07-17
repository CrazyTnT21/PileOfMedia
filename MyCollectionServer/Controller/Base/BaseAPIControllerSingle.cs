using Application.Controller;
using Microsoft.AspNetCore.Mvc;
using MyCollectionServer.CrudAPI;
using MySqlConnector;

namespace MyCollectionServer.Controller.Base;

public abstract class BaseAPIControllerSingle<T> : Base<T>, IBaseController<T>, ICrudAPI<T> where T : new()
{
  protected BaseAPIControllerSingle(ILogger logger, MySqlConnection connection) : base(logger, connection)
  {
  }

  [NonAction]
  public abstract Task<T?> GetSingle(uint id);

  [NonAction]
  public abstract Task<IEnumerable<T>> Get();

  [NonAction]
  public abstract Task<T> Create(T item);

  [NonAction]
  public abstract Task<T> Update(T item);

  [NonAction]
  public abstract Task Delete(uint id);

  [NonAction]
  public abstract Task Delete(uint[] ids);

  [HttpGet("{id}")]
  public abstract Task<ActionResult<T?>> GetSingleResult(uint id);

  [HttpGet]
  public abstract Task<ActionResult<IEnumerable<T>>> GetResult();

  [HttpPost]
  public abstract Task<ActionResult<T>> CreateResult(T item);

  [HttpPut]
  public abstract Task<ActionResult<T>> UpdateResult(T item);

  [HttpDelete("{id}")]
  public abstract Task<IActionResult> DeleteResult(uint id);

  [HttpDelete]
  public abstract Task<IActionResult> DeleteResult(uint[] ids);
}
