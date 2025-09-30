// Node.js usage example
import { NodeClient } from '@taskqueue/sdk';

async function main() {
  const client = new NodeClient({
    baseUrl: 'http://localhost:16080',
    apiKey: 'your-api-key',
  });

  try {
    // Create a task
    const task = await client.createTask({
      name: 'Process Data',
      command: 'python process.py',
      project_id: 'your-project-id',
      priority: 'High',
    });

    console.log('Created task:', task);

    // List tasks
    const tasks = await client.listTasks({
      status: 'Pending',
      limit: 10,
    });

    console.log('Tasks:', tasks);

    // Connect to WebSocket for real-time updates
    await client.connectWebSocket();
    console.log('Connected to WebSocket');

  } catch (error) {
    console.error('Error:', error);
  }
}

main();
