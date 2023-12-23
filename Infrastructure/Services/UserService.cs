using Application.Repositories;
using Domain.Classes;
using Domain.Services;
using Domain.ValueObjects;

namespace Infrastructure.Services;

public sealed class UserService : Service, IUserService
{
  private readonly IUserRepository _repository;
  private readonly IImageService _imageService;

  public UserService(IUserRepository repository, IImageService imageService)
  {
    _repository = repository;
    _imageService = imageService;
  }

  public async Task<IResult<User?>> GetById(uint id)
  {
    return new Ok<User?>(await _repository.GetById(id));
  }

  public async Task<IEnumerable<User>> Get()
  {
    var result = await _repository.Get();
    return result;
  }

  public async Task<IResult<User?>> GetByIdDeleted(uint id)
  {
    return new Ok<User?>(await _repository.GetById(id));
  }

  public async Task<IResult<IEnumerable<User>>> GetDeleted()
  {
    var result = await _repository.Get();
    return new Ok<IEnumerable<User>>(result);
  }

  public async Task<IResult<User>> Create(CreateUser item)
  {
    if (item.ProfilePicture is not null)
    {
      return await _imageService.Create(item.ProfilePicture) switch
      {
        Fail<Image> fail => new Fail<User>(fail),
        Ok<Image> image => new Ok<User>(await Create(item, image)),
        _ => throw new ArgumentException()
      };
    }

    return new Ok<User>(await _repository.Create(item));
  }

  public async Task<User> Create(CreateUser item, Image image)
  {
    item.ProfilePicture!.Id = image.Id;
    return await _repository.Create(item);
  }

  public async Task<IResult<User>> Update(CreateUser item)
  {
    return new Ok<User>(await _repository.Update(item));
  }

  public async Task<IResult> Delete(uint id)
  {
    await _repository.Delete(id);
    return new Ok();
  }

  public IResult Validate(CreateUser item)
  {
    if (item.ProfilePicture is null)
      return new Ok();

    var result = _imageService.Validate(item.ProfilePicture);
    if (result.Failure)
      return result;

    return new Ok();
  }
}
