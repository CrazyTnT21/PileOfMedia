using Application.Crud;
using Domain.Classes;

namespace Application.Controller;

public interface IBookService : ICrud<Book, BookGetOption, BookCreateOption, BookUpdateOption>
{
}

public sealed class BookGetOption
{
  public string Language;

  public BookGetOption(string language)
  {
    Language = language;
  }
}

public sealed class BookCreateOption
{
}

public sealed class BookUpdateOption
{
}
