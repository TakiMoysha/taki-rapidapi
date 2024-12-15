import json

from granian.rsgi import Scope as RSGIScope
from typing import Dict, Iterable, Mapping

 
class RSGIApplication:
    async def __rsgi__(self, scope: RSGIScope, proto):
        ...

    def __rsgi_init__(self, loop):
        ...

    def __rsgi_del__(self, loop):
        ...

class RSGIResponse:
    """A low-lever RSGI response class, with stubs and defaults, oriented only for prototyping (JSON)."""

    __slots__ = (
        "body",
        "content_length",
        "headers",
        "status_code",
    )

    def __init__(
        self,
        *,
        body: bytes | str = b"",
        status_code: int | None = None,
        content_length: int | None = None,
        headers: Dict[str, str] | None = None,
    ) -> None:
        if content_length is None:
            content_length = len(body)

        content_type = "application/json"
        charset = "utf-8"
        self.headers.setdefault("content-type", (f"{content_type}; charset={charset}"))
        self.body = body.encode() if isinstance(body, str) else body
        self.content_length = content_length
        self.headers = headers or {}
        self.status_code = status_code or 200

    async def start_response(self, send) -> None:
        event = {
            "type": "http.response.start",
            "status": self.status_code,
            "headers": [(k.encode(), v.encode()) for k, v in self.headers.items()],
        }
        await send(event)


async def rsgi_app(scope: RSGIScope, proto):
    assert scope.proto == "http"

    proto.response_str(
        status=200,
        headers=[
            ("Content-Type", "application/json; charset=utf-8"),
        ],
        body=json.dumps({"hello": "world"}),
    )
