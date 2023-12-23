using Domain.Error;

namespace Domain.ValueObjects;

public interface IResult
{
  public bool Failure { get; }
  public bool Success { get; }
}

public interface IResult<T>
{
  public bool Failure { get; }
  public bool Success { get; }
}

public readonly struct Ok<T> : IResult<T>
{
  public Ok(T item)
  {
    Value = item;
    Failure = false;
  }

  public readonly T Value;
  public bool Failure { get; }
  public bool Success => !Failure;

  public static implicit operator T(Ok<T> item) => item.Value;
  public static implicit operator Ok<T>(T item) => new(item);
}

public readonly struct Ok : IResult
{
  public Ok()
  {
    Failure = false;
  }

  public bool Failure { get; }
  public bool Success => !Failure;
}

public readonly struct Fail<T> : IResult<T>
{
  public Fail(ErrorMessage item)
  {
    ErrorMessage = item;
    Failure = true;
  }

  public readonly ErrorMessage ErrorMessage;
  public bool Failure { get; }
  public bool Success => !Failure;

  public static implicit operator ErrorMessage(Fail<T> item) => item.ErrorMessage;
  public static implicit operator Fail<T>(ErrorMessage item) => new(item);
  public static implicit operator Fail<T>(Fail item) => new(item);
  public static implicit operator Fail(Fail<T> item) => new(item);
}

public readonly struct Fail : IResult
{
  public Fail(ErrorMessage item)
  {
    ErrorMessage = item;
    Failure = true;
  }

  public readonly ErrorMessage ErrorMessage;
  public bool Failure { get; }
  public bool Success => !Failure;

  public static implicit operator ErrorMessage(Fail item) => item.ErrorMessage;
  public static implicit operator Fail(ErrorMessage item) => new(item);

  public Fail<T> F<T>() => new Fail<T>();
}
