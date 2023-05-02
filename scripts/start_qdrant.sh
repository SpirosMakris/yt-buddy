#!/bin/sh
echo "Pulling latest qdrant.."
docker pull qdrant/qdrant

echo "Starting local Qdrant.."
docker run -p 6333:6333 -p 6334:6334 \
    -e QDRANT__SERVICE__GRPC_PORT="6334" \
    -v $(pwd)/qdrant_storage:/qdrant/storage \
    qdrant/qdrant
