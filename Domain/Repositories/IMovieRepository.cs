using Application.Repositories;
using Domain.Classes;
using Domain.Enums;

namespace Domain.Repositories;

public interface IMovieRepository : IRepository
{
  public Task<Movie?> GetById(uint id, Language language);
  public Task<IEnumerable<Movie>> Get(Language language);
  public Task<IEnumerable<uint>> GetUserIdsByMovie(uint id);
  public Task<IEnumerable<User>> GetUsersByMovie(uint id);
  public Task<Movie> Create(CreateMovie item);
  public Task<Movie> Update(CreateMovie item);
  public Task Delete(uint id);
}
