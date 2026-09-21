@echo off
setlocal enabledelayedexpansion

for /f "tokens=1,* delims==" %%A in (.env) do (
    if "%%A"=="DATABASE_URL" set DATABASE_URL=%%B
)

sqlx migrate run --database-url "%DATABASE_URL%"

pause