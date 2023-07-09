using System.Collections.Generic;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Mvc;

namespace MyCollectionServer.Core;

public interface ICrud<T> : IGet<T>, ICreate<T>, IUpdate<T>, IDelete
{
}

public interface ICrudSingle<T> : IGetSingle<T>, ICreate<T>, IUpdate<T>, IDeleteSingle
{
}

public interface IGet<T> : IGetSingle<T>
{
  public abstract Task<ActionResult<List<T>>> Get(uint? start, uint? limit, string? orderColumn, Order? order);
}

public interface IGetSingle<T>
{
  public abstract Task<ActionResult<T?>> GetSingle(uint id);
}

public interface ICreate<in T>
{
  public abstract Task<ActionResult<long>> Create(T item);
}

public interface IUpdate<in T>
{
  public abstract Task<IActionResult> Update(T item);
}

public interface IDelete : IDeleteSingle
{
  public abstract Task<IActionResult> Delete(uint[] ids);
}

public interface IDeleteSingle
{
  public abstract Task<IActionResult> Delete(uint id);
}
