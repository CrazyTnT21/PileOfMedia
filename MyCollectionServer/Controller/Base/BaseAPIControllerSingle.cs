using System.Collections.Generic;
using System.Threading.Tasks;
using Application.Controller;
using Application.Crud;
using Microsoft.Extensions.Logging;
using Infrastructure.EF;
using Microsoft.AspNetCore.Mvc;
using Microsoft.EntityFrameworkCore;
using MyCollectionServer.CrudAPI;

namespace MyCollectionServer.Controller.Base;

public abstract class BaseAPIControllerSingle<T>: IBaseController<T>, ICrudAPI<T>
{
  protected readonly ILogger _logger;
  protected readonly DbContext _dbContext;

  public BaseAPIControllerSingle(ILogger logger, AppDBContext dbContext)
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

  public abstract Task<ActionResult<T?>> GetSingleResult(uint id);
  public abstract Task<ActionResult<IEnumerable<T>>> GetResult();
  public abstract Task<ActionResult<uint>> CreateResult(T item);
  public abstract Task<IActionResult> UpdateResult(T item);
  public abstract Task<IActionResult> DeleteResult(uint id);
  public abstract Task<IActionResult> DeleteResult(uint[] ids);
}
