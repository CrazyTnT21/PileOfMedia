namespace Domain.Interfaces;

public interface IDateTimeProvider
{
  DateTime Now { get; }
  DateOnly Today { get; }
}
