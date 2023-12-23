using Domain.Classes;
using Domain.Enums;

namespace Application.Repositories;

public interface IAccountRepository : IRepository
{
  public Task<Account?> GetById(uint id);
  public Task<IEnumerable<Account>> Get();
  public Task<Account> Create(CreateAccount item);
  public Task<Account> Update(CreateAccount item);
  public Task Delete(uint id);
}
