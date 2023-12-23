using Domain.Classes;
using Domain.Common;
using Domain.ValueObjects;

namespace Domain.Services;

public interface IAccountService
{
  public Task<IResult<Account?>> GetById(uint id);
  public Task<IResult<IEnumerable<Account>>> Get();
  public Task<IResult<Account>> Create(CreateAccount item);
  public Task<IResult<Account>> Update(CreateAccount item);
  public Task<IResult> Delete(uint id);
  public IResult Validate(CreateAccount item);
}
