from app import app
from flask import make_response


@app.route("/error-assert-query-cookie")
def error_assert_query_cookie():
    resp = make_response()
    resp.set_cookie("cookie1", "value1")
    resp.set_cookie("cookie2", "value2", secure=True)
    return resp
