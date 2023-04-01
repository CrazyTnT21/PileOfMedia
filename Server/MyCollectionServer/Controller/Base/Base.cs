using Application.Crud;

namespace MyCollectionServer.Controller.Base;

public abstract class Base<T> : IValidate<T> where T : new()
{
  public abstract void Validate(T item, bool update = false);
}
