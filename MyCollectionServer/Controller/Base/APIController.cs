using Application.Crud;
using Domain;
using Domain.Interfaces;
using Microsoft.AspNetCore.Mvc;
using MySqlConnector;

namespace MyCollectionServer.Controller.Base;

[ApiController]
[Route("[controller]")]
public abstract class APIController<T, TValidateOption> : Base<T>, IValidate<T, TValidateOption>
  where T : IEntity, new()
{
  protected APIController(ILogger logger, MySqlConnection connection) : base(logger, connection)
  {
  }

  public abstract HTTPError? Validate(T item, TValidateOption options);
}
