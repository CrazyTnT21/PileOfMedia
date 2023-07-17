using Application.Crud;
using Domain;
using Microsoft.AspNetCore.Mvc;
using MySqlConnector;

namespace MyCollectionServer.Controller.Base;

public abstract class Base<T> : IValidate<T> where T : new()
{
  protected readonly MySqlConnection _connection;
  protected readonly ILogger _logger;

  protected Base(ILogger logger, MySqlConnection connection)
  {
    _logger = logger;
    _connection = connection;
  }

  [NonAction]
  public abstract HTTPError? Validate(T item, bool update = false);
}
