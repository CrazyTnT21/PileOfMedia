using Domain.Classes;
using Domain.Enums;

namespace Application.Repositories;

public interface IUserRepository : IRepository
{
  public Task<User?> GetById(uint id);
  public Task<IEnumerable<User>> Get();
  public Task<User?> GetByIdDeleted(uint id);
  public Task<IEnumerable<User>> GetDeleted();
  public Task<User> Create(CreateUser item);
  public Task<User> Update(CreateUser item);
  public Task Delete(uint id);
}
