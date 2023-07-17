namespace Application.Crud;

public interface ICrud<T> : IGet<T>, ICreate<T>, IUpdate<T>, IDelete
{
}

public interface ICrudSingle<T> : IGetSingle<T>, ICreate<T>, IUpdate<T>, IDeleteSingle
{
}

public interface IGet<T> : IGetSingle<T>
{
  public Task<IEnumerable<T>> Get();
}

public interface IGetSingle<T>
{
  public Task<T?> GetSingle(uint id);
}

public interface ICreate<T>
{
  public Task<T> Create(T item);
}

public interface IUpdate<T>
{
  public Task<T> Update(T item);
}

public interface IDelete : IDeleteSingle
{
  public Task Delete(uint[] ids);
}

public interface IDeleteSingle
{
  public Task Delete(uint id);
}
