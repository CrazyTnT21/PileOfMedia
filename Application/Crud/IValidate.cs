using Domain;

namespace Application.Crud;

public interface IValidate<T, TOption>
{
  public HTTPError? Validate(T item, TOption options);
}
