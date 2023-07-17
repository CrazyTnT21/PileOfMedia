using System.Net;
using Microsoft.AspNetCore.Http;

namespace Domain.Exceptions;

public class HTTPException : Exception
{
  public readonly int StatusCode;
  public readonly string? Reason;

  public HTTPException() : this(StatusCodes.Status500InternalServerError)
  {
  }

  public HTTPException(string reason, Exception? inner = null)
    : this(StatusCodes.Status500InternalServerError, reason, inner)
  {
  }

  public HTTPException(HttpStatusCode statusCode, string? reason = null, Exception? inner = null)
    : this((int)statusCode, reason, inner)
  {
  }

  public HTTPException(int statusCode, string? reason = null, Exception? inner = null)
    : base(reason, inner)
  {
    StatusCode = statusCode;
    Reason = reason;
  }
}
