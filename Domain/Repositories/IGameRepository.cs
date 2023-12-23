using Domain.Classes;
using Domain.Enums;

namespace Application.Repositories;

public interface IGameRepository: IRepository
{
  public Task<Game?> GetById(uint id,Language language);
  public Task<IEnumerable<Game>> Get(Language language);
  public Task<Game> Create(CreateGame item);
  public Task<Game> Update(CreateGame item);
  public Task Delete(uint id);
}
