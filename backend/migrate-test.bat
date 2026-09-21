@echo off
setlocal enabledelayedexpansion

for /f "tokens=1,* delims==" %%A in (.env) do (
    if "%%A"=="DATABASE_URL_TEST" set DATABASE_URL_TEST=%%B
)

sqlx migrate run --database-url "%DATABASE_URL_TEST%"

pause