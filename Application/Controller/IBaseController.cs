using Application.Crud;

namespace Application.Controller;

public interface IBaseController<T> : ICrud<T>, IValidate<T>
{
}
