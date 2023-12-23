using Application.Repositories;
using Domain.Common;
using Domain.Interfaces;
using Domain.Repositories;
using Domain.Services;
using Infrastructure.Services;

namespace MyCollectionServer;

public static class Implementation
{
  public static void DefineImplementations(this WebApplicationBuilder builder)
  {
    builder.Services.AddSingleton<IDateTimeProvider, DateTimeProvider>();
    
    builder.Services.AddSingleton<IGenreService, GenreService>();
    builder.Services.AddSingleton<IGenreRepository, GenreRepository>();

    builder.Services.AddSingleton<IImageService, ImageService>();
    builder.Services.AddSingleton<IImageRepository, ImageRepository>();

    builder.Services.AddSingleton<IMovieService, MovieService>();
    builder.Services.AddSingleton<IMovieRepository, MovieRepository>();

    builder.Services.AddSingleton<IGraphicNovelService, GraphicNovelService>();
    builder.Services.AddSingleton<IGraphicNovelRepository, GraphicNovelRepository>();

    builder.Services.AddSingleton<IGameService, GameService>();
    builder.Services.AddSingleton<IGameRepository, GameRepository>();

    builder.Services.AddSingleton<IBookService, BookService>();
    builder.Services.AddSingleton<IBookRepository, BookRepository>();

    builder.Services.AddSingleton<IUserService, UserService>();
    builder.Services.AddSingleton<IUserRepository, UserRepository>();

    builder.Services.AddSingleton<IAccountService, AccountService>();
    builder.Services.AddSingleton<IAccountRepository, AccountRepository>();
  }
}
