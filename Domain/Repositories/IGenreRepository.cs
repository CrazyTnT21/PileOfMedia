using Domain.Classes;
using Domain.Enums;

namespace Application.Repositories;

public interface IGenreRepository : IRepository
{
  public Task<Genre?> GetById(uint id, Language language);
  public Task<Genre?> GetByName(string name, Language language);
  public Task<IEnumerable<Genre>> Get(Language language);
  public Task<Genre> Create(CreateGenre item);
  public Task<Genre> Update(CreateGenre item);
  public Task Delete(uint id);
}
