using Domain.Classes;
using Domain.ValueObjects;

namespace Domain.Services;

public interface IUserService
{
  public Task<IResult<User?>> GetById(uint id);
  public Task<IEnumerable<User>> Get();
  public Task<IResult<User?>> GetByIdDeleted(uint id);
  public Task<IResult<IEnumerable<User>>> GetDeleted();
  public Task<IResult<User>> Create(CreateUser item);
  public Task<IResult<User>> Update(CreateUser item);
  public Task<IResult> Delete(uint id);
  public IResult Validate(CreateUser item);
}
