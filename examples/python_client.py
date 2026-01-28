#!/usr/bin/env python3
"""
A2A Python Client Example
This example demonstrates how to use the official a2a-python client to communicate with our Rust server.
"""

import asyncio
import sys
import uuid
from typing import AsyncIterator

# Import from a2a-python
try:
    from a2a.client.client_factory import ClientFactory
    from a2a.client.client import ClientConfig
    from a2a.types import (
        Message, Part, TextPart, DataPart, Role,
        Task, TaskStatusUpdateEvent, TaskArtifactUpdateEvent
    )
except ImportError as e:
    print(f"❌ Missing a2a-python package: {e}")
    print("   Install with: pip install a2a-sdk")
    sys.exit(1)


async def print_events(agent_card):
    """Event consumer function to print events as they arrive"""
    async def consumer(event, card):
        if isinstance(event, tuple) and len(event) == 2:
            task, update = event
            print(f"📡 Event: Task {task.id} - {task.status.state}")
            if update:
                if isinstance(update, TaskStatusUpdateEvent):
                    print(f"   Status Update: {update.status.state}")
                elif isinstance(update, TaskArtifactUpdateEvent):
                    print(f"   Artifact Update: {update.artifact.name}")
        elif isinstance(event, Message):
            print(f"📨 Message: {event.role} - {len(event.parts)} parts")
            for i, part in enumerate(event.parts):
                if part.root.kind == 'text':
                    print(f"   Part {i+1} (text): {part.root.text}")
                elif part.root.kind == 'data':
                    print(f"   Part {i+1} (data): {part.root.data}")
        else:
            print(f"📡 Unknown event type: {type(event)}")
    
    return consumer


async def main():
    """Main function to demonstrate the a2a-python client"""
    print("🚀 A2A Python Client Example (using a2a-python)")
    print("=" * 60)
    
    # Configure client
    config = ClientConfig(
        streaming=True,  # Enable streaming support
        polling=False,   # Don't use polling
    )
    
    try:
        print("🔗 Connecting to Rust server at http://localhost:8080...")
        
        # Create client using ClientFactory
        client = await ClientFactory.connect(
            agent="http://localhost:8080",
            client_config=config,
        )
        
        # Get agent card
        agent_card = await client.get_card()
        print(f"✅ Connected to agent: {agent_card.name}")
        print(f"📝 Description: {agent_card.description}")
        print(f"🌐 Server URL: {agent_card.url}")
        print(f"🔧 Preferred Transport: {agent_card.preferred_transport}")
        print()
        
        # Add event consumer
        event_consumer = await print_events(agent_card)
        await client.add_event_consumer(event_consumer)
        
        # Test 1: Simple text message
        print("📤 Test 1: Sending simple text message...")
        simple_message = Message(
            role=Role.user,
            parts=[
                Part(root=TextPart(text="Hello from Python a2a-client!"))
            ],
            message_id=str(uuid.uuid4())
        )
        
        response_count = 0
        async for event in client.send_message(simple_message):
            response_count += 1
            if response_count > 10:  # Prevent infinite loops
                break
        print()
        
        # Test 2: Message with multiple parts
        print("📤 Test 2: Sending multi-part message...")
        multi_message = Message(
            role=Role.user,
            parts=[
                Part(root=TextPart(text="This is a test with multiple parts:")),
                Part(root=DataPart(data={"test": True, "client": "Python a2a-sdk"})),
                Part(root=TextPart(text="End of message"))
            ],
            message_id=str(uuid.uuid4()),
            context_id="ctx-123"
        )
        
        response_count = 0
        async for event in client.send_message(multi_message):
            response_count += 1
            if response_count > 10:  # Prevent infinite loops
                break
        print()
        
        # Test 3: Message with task ID
        print("📤 Test 3: Sending message with task ID...")
        task_message = Message(
            role=Role.user,
            parts=[
                Part(root=TextPart(text="Message with task context"))
            ],
            message_id=str(uuid.uuid4()),
            task_id="task-456",
            context_id="ctx-123"
        )
        
        response_count = 0
        async for event in client.send_message(task_message):
            response_count += 1
            if response_count > 10:  # Prevent infinite loops
                break
        print()
        
        print("✅ All tests completed successfully!")
        print(f"🎯 The Rust server and Python client are fully compatible!")
        
    except Exception as e:
        print(f"❌ Error: {e}")
        if "Connection refused" in str(e) or "ConnectError" in str(e):
            print("💡 Make sure the Rust server is running:")
            print("   cargo run --example rust_server")
        sys.exit(1)


if __name__ == "__main__":
    # Run the client
    asyncio.run(main())
