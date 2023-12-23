using Application.Repositories;
using Domain.Classes;
using Domain.Services;
using Domain.ValueObjects;

namespace Infrastructure.Services;

public sealed class AccountService : Service, IAccountService
{
  private readonly IAccountRepository _repository;
  private readonly IUserService _userService;

  public AccountService(IAccountRepository repository, IUserService userService)
  {
    _repository = repository;
    _userService = userService;
  }

  public async Task<IResult<Account?>> GetById(uint id)
  {
    return new Ok<Account?>(await _repository.GetById(id));
  }

  public async Task<IResult<IEnumerable<Account>>> Get()
  {
    var result = await _repository.Get();
    return new Ok<IEnumerable<Account>>(result);
  }

  public async Task<IResult<Account>> Create(CreateAccount item)
  {
    if (Validate(item) is Fail invalid)
      return new Fail<Account>(invalid);

    var userResult = await _userService.Create(item.User);

    if (userResult is Fail<User> userFail)
      return new Fail<Account>(userFail);

    User user = (Ok<User>)userResult;
    item.User.Id = user.Id;

    return new Ok<Account>(await _repository.Create(item));
  }

  public async Task<IResult<Account>> Update(CreateAccount item)
  {
    if (Validate(item) is Fail invalid)
      return new Fail<Account>(invalid);

    return new Ok<Account>(await _repository.Update(item));
  }

  public async Task<IResult> Delete(uint id)
  {
    await _repository.Delete(id);
    return new Ok();
  }

  public IResult Validate(CreateAccount item)
  {
    return new Ok();
  }
}
