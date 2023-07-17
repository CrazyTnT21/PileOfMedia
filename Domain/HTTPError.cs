using System.Net;
using Microsoft.AspNetCore.Http;

namespace Domain;

public struct HTTPError
{
  public readonly int StatusCode;
  public readonly string? Reason;

  public HTTPError() : this(StatusCodes.Status500InternalServerError)
  {
  }

  public HTTPError(string reason, Exception? inner = null)
    : this(StatusCodes.Status500InternalServerError, reason, inner)
  {
  }

  public HTTPError(HttpStatusCode statusCode, string? reason = null, Exception? inner = null)
    : this((int)statusCode, reason, inner)
  {
  }

  public HTTPError(int statusCode, string? reason = null, Exception? innerException = null)
  {
    StatusCode = statusCode;
    Reason = reason;
  }
}
