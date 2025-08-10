FROM python:3.11-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential gcc patchelf libxml2-dev libxslt1-dev zlib1g-dev libjpeg62-turbo-dev libpng-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY requirements.txt ./requirements.txt
RUN python -m pip install --upgrade pip \
 && python -m pip install --no-cache-dir -r requirements.txt \
 && python -m pip install --no-cache-dir pyinstaller

COPY cxbin_converter/ ./cxbin_converter/
COPY icon.ico ./icon.ico

ENV ENTRY=./cxbin_converter/cxbin_converter.py
ENV NAME=cxbin_converter

CMD bash -lc '\
  if [ -f "icon.ico" ]; then \
    pyinstaller --icon=icon.ico --onefile --name "$NAME" "$ENTRY"; \
  else \
    pyinstaller --onefile --name "$NAME" "$ENTRY"; \
  fi; \
  mkdir -p /out && cp -a dist/* /out/ \
'
