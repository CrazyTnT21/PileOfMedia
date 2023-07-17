using Domain;

namespace Application.Crud;

public interface IValidate<in T>
{
  public HTTPError? Validate(T item, bool update = false);
}
