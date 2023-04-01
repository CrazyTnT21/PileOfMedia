using Domain.Classes;
using Microsoft.EntityFrameworkCore;

namespace Infrastructure.EF;

public class AppDBContext: DbContext
{

  public AppDBContext(DbContextOptions<AppDBContext> options) : base(options)
  {
  }

  public DbSet<Comic> Comics { get; set; }
}
