namespace Application.Crud;

public interface ICrud<T, TGetOptions, TCreateOptions, TUpdateOptions> : IGet<T, TGetOptions>,
  ICreate<T, TCreateOptions>, IUpdate<T, TUpdateOptions>, IDelete
{
}

public interface ICrudSingle<T, TGetOptions, TCreateOptions, TUpdateOptions> : IGetSingle<T, TGetOptions>,
  ICreate<T, TCreateOptions>, IUpdate<T, TUpdateOptions>, IDeleteSingle
{
}

public interface IGet<T, TOption> : IGetSingle<T, TOption>
{
  public Task<IEnumerable<T>> Get(TOption options);
}

public interface IGetSingle<T, TOption>
{
  public Task<T?> GetSingle(uint id, TOption options);
}

public interface ICreate<T, TOption>
{
  public Task<T> Create(T item, TOption options);
}

public interface IUpdate<T, TOption>
{
  public Task<T> Update(T item, TOption options);
}

public interface IDelete : IDeleteSingle
{
  public Task Delete(uint[] ids);
}

public interface IDeleteSingle
{
  public Task Delete(uint id);
}
