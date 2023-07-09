using Application.Controller;
using Application.Crud;
using Infrastructure.EF;
using Microsoft.AspNetCore.Mvc;
using Microsoft.EntityFrameworkCore;
using MyCollectionServer.CrudAPI;

namespace MyCollectionServer.Controller.Base;

public abstract class BaseAPIController<T>: IBaseController<T>, ICrudAPI<T>
{
  protected readonly ILogger _logger;
  protected readonly AppDBContext _dbContext;

  public BaseAPIController(ILogger logger, AppDBContext dbContext)
  {
    _logger = logger;
    _dbContext = dbContext;
  }
  public abstract Task<T?> GetSingle(uint id);
  public abstract Task<IEnumerable<T>> Get();
  public abstract Task<uint> Create(T item);
  public abstract Task Update(T item);
  public abstract Task Delete(uint id);
  public abstract Task Delete(uint[] ids);
  public abstract void Validate(T item, bool update = false);

  [HttpGet("{id}")]
  public abstract Task<ActionResult<T?>> GetSingleResult(uint id);
  [HttpGet]
  public abstract Task<ActionResult<IEnumerable<T>>> GetResult();
  [HttpPost]
  public abstract Task<ActionResult<uint>> CreateResult(T item);
  [HttpPut]
  public abstract Task<IActionResult> UpdateResult(T item);
  [HttpDelete("{id}")]
  public abstract Task<IActionResult> DeleteResult(uint id);
  [HttpDelete]
  public abstract Task<IActionResult> DeleteResult(uint[] ids);
}
