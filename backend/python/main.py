from fastapi import FastAPI, HTTPException, status
from fastapi.middleware.cors import CORSMiddleware
from contextlib import asynccontextmanager
import uvicorn
import os
import asyncio
import redis.asyncio as redis
from dotenv import load_dotenv
from datetime import datetime
import json
from typing import Optional, Dict, Any

# Load environment variables
load_dotenv()

# Configuration
class Settings:
    port: int = int(os.getenv("PORT", "8001"))
    database_url: str = os.getenv("DATABASE_URL", "postgresql://olympus:devpassword@localhost:5432/olympus")
    redis_url: str = os.getenv("REDIS_URL", "redis://localhost:6379")
    environment: str = os.getenv("ENVIRONMENT", "development")

settings = Settings()

# Global variables
redis_client: Optional[redis.Redis] = None
redis_subscriber: Optional[asyncio.Task] = None

@asynccontextmanager
async def lifespan(app: FastAPI):
    """Startup and shutdown events"""
    global redis_client, redis_subscriber

    # Startup
    print("🚀 Starting Python Analytics Service...")

    # Connect to Redis
    try:
        redis_client = redis.from_url(settings.redis_url)
        await redis_client.ping()
        print("✅ Connected to Redis")

        # Start event subscriber
        redis_subscriber = asyncio.create_task(subscribe_to_events())
        print("✅ Started Redis event subscriber")
    except Exception as e:
        print(f"⚠️ Could not connect to Redis: {e}")

    yield

    # Shutdown
    print("👋 Shutting down...")
    if redis_subscriber:
        redis_subscriber.cancel()
    if redis_client:
        await redis_client.close()

# Create FastAPI app
app = FastAPI(
    title="Olympus Analytics Service",
    description="AI/ML and Analytics Service",
    version="1.0.0",
    lifespan=lifespan
)

# CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["http://localhost:3000", "http://localhost:8080"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Redis event subscriber
async def subscribe_to_events():
    """Subscribe to Redis events"""
    if not redis_client:
        return

    pubsub = redis_client.pubsub()
    await pubsub.subscribe(
        "events.user.logged_in",
        "events.user.created"
    )

    print("📡 Listening for events...")

    async for message in pubsub.listen():
        if message["type"] == "message":
            try:
                data = json.loads(message["data"])
                print(f"📊 Event: {message['channel']}")
            except Exception as e:
                print(f"Error: {e}")

@app.get("/health")
async def health_check():
    """Health check endpoint"""
    return {
        "status": "healthy",
        "service": "python-analytics",
        "timestamp": datetime.utcnow()
    }

@app.get("/analytics/dashboard/{tenant_id}")
async def dashboard(tenant_id: str):
    """Get dashboard data"""
    return {
        "tenant_id": tenant_id,
        "metrics": {
            "users": 150,
            "revenue": 5280.50,
            "orders": 32
        }
    }

if __name__ == "__main__":
    uvicorn.run(
        "main:app",
        host="0.0.0.0",
        port=settings.port,
        reload=True
    )
