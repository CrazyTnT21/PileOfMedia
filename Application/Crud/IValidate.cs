namespace Application.Crud;

public interface IValidate<in T>
{
  //Used for validating whether an item can be inserted/updated
  public void Validate(T item, bool update = false);
}
