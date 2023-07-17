using System.ComponentModel.DataAnnotations.Schema;
using System.Reflection;
using Application.Controller;
using Domain.Interfaces;
using Microsoft.AspNetCore.Mvc;
using MySqlConnector;

namespace MyCollectionServer.Controller.Base;

[ApiController]
[Route("[controller]")]
public abstract class BaseAPIController<T> : Base<T>, IBaseController<T> where T : IEntity, new()
{
  protected BaseAPIController(ILogger logger, MySqlConnection connection) : base(logger, connection)
  {
  }

  public virtual async Task<T?> GetSingle(uint id)
  {
    throw new NotImplementedException();
  }

  public virtual async Task<IEnumerable<T>> Get()
  {
    throw new NotImplementedException();
  }

  public virtual async Task<T> Create(T item)
  {
    throw new NotImplementedException();
  }

  public virtual async Task<T> Update(T item)
  {
    throw new NotImplementedException();
  }

  public virtual async Task Delete(uint id)
  {
    throw new NotImplementedException();
  }

  public virtual async Task Delete(uint[] ids)
  {
    throw new NotImplementedException();
  }

  public static List<string> GetColumns() => GetColumns(typeof(T));

  public static List<string> GetColumns<A>() => GetColumns(typeof(A));

  public static List<string> GetColumns(Type type)
  {
    var result = new List<string>();
    var properties = type.GetProperties();
    for (int i = 0; i < properties.Length; i++)
    {
      var column = properties[i].GetCustomAttribute<ColumnAttribute>();
      result.Add(column?.Name ?? properties[i].Name);
    }

    return result;
  }
}
