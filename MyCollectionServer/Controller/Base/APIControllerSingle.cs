using Application.Crud;
using Domain;
using Domain.Interfaces;
using MySqlConnector;

namespace MyCollectionServer.Controller.Base;

public abstract class APIControllerSingle<T, TValidateOption> : Base<T>, IValidate<T, TValidateOption>
  where T : IEntity, new()
{
  protected APIControllerSingle(ILogger logger, MySqlConnection connection) : base(logger, connection)
  {
  }

  public abstract HTTPError? Validate(T item, TValidateOption options);
}
