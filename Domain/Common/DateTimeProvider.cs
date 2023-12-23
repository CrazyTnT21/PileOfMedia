using Domain.Interfaces;

namespace Domain.Common;

public class DateTimeProvider : IDateTimeProvider
{
  public DateTime Now => DateTime.Now;
  public DateOnly Today => DateOnly.FromDateTime(DateTime.Now);
}
