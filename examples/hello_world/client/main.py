# -*- coding: utf-8 -*-
import asyncio
import httpx
import time as timer
from typing import Any, TypedDict, Literal

HelloQuery = TypedDict(
    "HelloQuery",
    {
        "language": Literal["en", "es", "zh"],
        "timeout": float | None,
        "async": bool,
    },
)
"""
The query parameters for the hello endpoint.

.. note::
    This is the Python equivalent of the Rust declaration in `models.rs`.
"""

class HelloPayload(TypedDict):
    """
    The response payload for the hello endpoint.

    .. note::
        This is the Python equivalent of the Rust declaration in `models.rs`.
    """
    name: str
    age: int


class HelloResult(TypedDict):
    """
    The response result for the hello endpoint.

    .. note::
        This is the Python equivalent of the Rust declaration in `models.rs`.
    """
    greeting: str
    answer_id: int

class HelloResponse(TypedDict):
    """
    The response for the hello endpoint.

    .. note::
        This is the Python equivalent of `OutputResponse`.
    """
    ticket: str
    metadata: dict[str, Any]
    output: HelloResult

class HelloError(TypedDict):
    """
    The response error for the hello endpoint.

    .. note::
        This is the Python equivalent of `ErrorSchema`.
    """
    error: str
    status_code: int
    details: dict[str, Any]

async def request_sync(
    name: str,
    age: int,
    language: Literal["en", "es", "zh"],
    timeout: float | None = None,
    *,
    client: httpx.AsyncClient,
) -> tuple[Literal[200], HelloResponse] | tuple[int, HelloError]:
    """
    Make a synchronous request to the hello endpoint.
    """
    start_time = timer.perf_counter()
    print(f"Making blocking request for {name} in {language}...")
    response = await client.post(
        "http://localhost:7007/request",
        params={
            "language": language,
            "timeout": timeout,
            "async": False,
        },
        json={
            "name": name,
            "age": age,
        }
    )

    print(f"Blocking request returned {response.status_code} in {timer.perf_counter() - start_time:.2f} seconds")
    return (response.status_code, response.json())

async def request_async(
    name: str,
    age: int,
    language: Literal["en", "es", "zh"],
    timeout: float | None = None,
    *,
    client: httpx.AsyncClient,
) -> tuple[Literal[200], HelloResponse] | tuple[int, HelloError]:
    """
    Make an asynchronous request to the hello endpoint.
    """
    start_time = timer.perf_counter()
    print(f"Making asynchronous request for {name} in {language}...")
    ticket = await client.post(
        "http://localhost:7007/request",
        params={
            "language": language,
            "timeout": timeout,
            "async": True,
        },
        json={
            "name": name,
            "age": age,
        }
    )

    if ticket.status_code != 202:
        print(f"Failed to get ticket for async request due to {ticket.status_code}: {ticket.json()}")
        return (ticket.status_code, ticket.json())

    ticket_id = ticket.json()["ticket"]
    print(f"Got ticket {ticket_id} in {timer.perf_counter() - start_time:.2f} seconds")

    while True:
        response = await client.get(f"http://localhost:7007/retrieve", params={"ticket": ticket_id, "timeout": 5})
        if response.status_code == 408:
            print("Timeout while waiting for async response; will try again...")
            await asyncio.sleep(1)
            continue

        break

    print(f"Async request returned {response.status_code} in {timer.perf_counter() - start_time:.2f} seconds")
    return (response.status_code, response.json())

async def main(sync_count: int, async_count: int):
    async with httpx.AsyncClient(timeout=None) as client:
        coros = [
            request_sync(f"Worker #{i}", 18, "en", 180, client=client)
            for i in range(sync_count)
        ] + [
            request_async(f"Worker #{i}", 25, "en", 180, client=client)
            for i in range(async_count)
        ]

        results = await asyncio.gather(*coros)

        good_results: list[HelloResponse] = []
        bad_results = []
        for (status, result) in results:
            if status == 200:
                good_results.append(result)
            else:
                bad_results.append(result)

    print(f"Got {len(good_results)} good results and {len(bad_results)} bad results: {len(good_results) / (sync_count + async_count):.2%}")

    all_good_greetings = set([
        result["output"]["greeting"]
        for result in good_results
    ])
    print(f"Got {len(all_good_greetings):,} unique greetings.")

    return good_results, bad_results

if __name__ == "__main__":
    asyncio.run(main(50, 50))
