using Application.Crud;
using Domain.Classes;

namespace Application.Controller;

public interface IComicService : ICrud<Comic, ComicGetOption, ComicCreateOption, ComicUpdateOption>
{
}

public struct ComicGetOption
{
  public string Language;

  public ComicGetOption(string language)
  {
    Language = language;
  }
}

public struct ComicCreateOption
{
}

public struct ComicUpdateOption
{
}
