using Domain.Classes;
using Domain.ValueObjects;

namespace Domain.Services;

public interface IImageService
{
  public Task<IResult<Image?>> GetById(uint id);
  public Task<IResult<IEnumerable<Image>>> Get();
  public Task<IResult<Image>> Create(CreateImage item);
  public Task<IResult<Image>> Update(CreateImage item);
  public Task<IResult> Delete(uint id);
  public IResult Validate(CreateImage item);
}
