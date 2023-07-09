using System.Collections.Generic;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Mvc;

namespace MyCollectionServer.CrudAPI;


public interface ICrudAPI<T> : IGetAPI<T>, ICreateAPI<T>, IUpdateAPI<T>, IDeleteAPI
{
}

public interface ICrudAPISingle<T> : IGetAPISingle<T>, ICreateAPI<T>, IUpdateAPI<T>, IDeleteAPISingle
{
}

public interface IGetAPI<T> : IGetAPISingle<T>
{
  public Task<ActionResult<IEnumerable<T>>> GetResult();
}

public interface IGetAPISingle<T>
{
  public Task<ActionResult<T?>> GetSingleResult(uint id);
}

public interface ICreateAPI<in T>
{
  public Task<ActionResult<uint>> CreateResult(T item);
}

public interface IUpdateAPI<in T>
{
  public Task<IActionResult> UpdateResult(T item);
}

public interface IDeleteAPI : IDeleteAPISingle
{
  public Task<IActionResult> DeleteResult(uint[] ids);
}

public interface IDeleteAPISingle
{
  public Task<IActionResult> DeleteResult(uint id);
}

