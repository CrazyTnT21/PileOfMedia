using Application.Repositories;
using Domain.Classes;
using Domain.Services;
using Domain.ValueObjects;

namespace Infrastructure.Services;

public sealed class ImageService : Service, IImageService
{
  private IImageRepository _repository;

  public ImageService(IImageRepository repository)
  {
    _repository = repository;
  }

  public async Task<IResult<Image?>> GetById(uint id)
  {
    throw new NotImplementedException();
  }

  public async Task<IResult<IEnumerable<Image>>> Get()
  {
    throw new NotImplementedException();
  }

  public async Task<IResult<Image>> Create(CreateImage item)
  {
    throw new NotImplementedException();
  }

  public async Task<IResult<Image>> Update(CreateImage item)
  {
    throw new NotImplementedException();
  }

  public async Task<IResult> Delete(uint id)
  {
    throw new NotImplementedException();
  }

  public IResult Validate(CreateImage item)
  {
    throw new NotImplementedException();
  }
}
