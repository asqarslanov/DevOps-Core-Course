FROM python:3.14.2-slim-trixie

WORKDIR /app/

RUN apt-get update \
    && apt-get install --yes --no-install-recommends gcc build-essential \
    && rm --force --recursive /var/lib/apt/lists/*

COPY ./requirements.txt ./
RUN python -m pip install --upgrade pip \
    && python -m pip install --no-cache-dir --requirement=./requirements.txt \
    && rm --force --recursive /root/.cache/pip/

COPY ./app.py ./

RUN useradd --create-home --shell /bin/bash appuser
RUN chown -R appuser:appuser /app/
USER appuser

EXPOSE 5000

CMD ["python", "app.py"]
