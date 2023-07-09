using Application.Crud;

namespace Application.Controller;

public interface IBaseControllerSingle<T> : ICrudSingle<T>, IValidate<T>
{
}
