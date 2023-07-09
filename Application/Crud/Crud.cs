using System.Collections.Generic;
using System.Threading.Tasks;
using Domain.Enums;
using Microsoft.AspNetCore.Mvc;

namespace Application.Crud;

public interface ICrud<T> : IGet<T>, ICreate<T>, IUpdate<T>, IDelete
{
}

public interface ICrudSingle<T> : IGetSingle<T>, ICreate<T>, IUpdate<T>, IDeleteSingle
{
}

public interface IGet<T> : IGetSingle<T>
{
  public Task<IEnumerable<T>> Get();
}

public interface IGetSingle<T>
{
  public Task<T?> GetSingle(uint id);
}

public interface ICreate<in T>
{
  public Task<uint> Create(T item);
}

public interface IUpdate<in T>
{
  public Task Update(T item);
}

public interface IDelete : IDeleteSingle
{
  public Task Delete(uint[] ids);
}

public interface IDeleteSingle
{
  public Task Delete(uint id);
}
