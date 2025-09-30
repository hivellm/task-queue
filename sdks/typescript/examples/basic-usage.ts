// Task Queue TypeScript SDK - Basic Usage Example
import { TaskQueueClient } from '@taskqueue/sdk';

async function main() {
  // Initialize the client
  const client = new TaskQueueClient({
    baseUrl: 'http://localhost:16080',
    apiKey: 'your-api-key', // Optional
  });

  try {
    console.log('🚀 Task Queue SDK Example');
    console.log('========================');

    // Health check
    console.log('\n📊 Checking server health...');
    const health = await client.healthCheck();
    console.log('Health status:', health);

    // Create a project first
    console.log('\n📁 Creating project...');
    const project = await client.createProject({
      name: 'SDK Test Project',
      description: 'Project created by TypeScript SDK',
      tags: ['sdk', 'test'],
    });
    console.log('Created project:', project);

    // Create a task
    console.log('\n📝 Creating task...');
    const task = await client.createTask({
      name: 'Process Data',
      command: 'python process.py --input data.csv --output results.json',
      description: 'Process customer data and generate analytics report',
      project_id: project.id,
      priority: 'High',
      metadata: {
        environment: 'production',
        version: '1.0.0',
      },
    });
    console.log('Created task:', task);

    // List tasks
    console.log('\n📋 Listing tasks...');
    const tasks = await client.listTasks({
      project_id: project.id,
      limit: 10,
    });
    console.log(`Found ${tasks.length} tasks:`);
    tasks.forEach(t => console.log(`  - ${t.name} (${t.status})`));

    // Get task details
    console.log('\n🔍 Getting task details...');
    const taskDetails = await client.getTask(task.id);
    console.log('Task details:', {
      id: taskDetails.id,
      name: taskDetails.name,
      status: taskDetails.status,
      priority: taskDetails.priority,
      created_at: taskDetails.created_at,
    });

    // Update task
    console.log('\n✏️ Updating task...');
    const updatedTask = await client.updateTask(task.id, {
      description: 'Updated description via SDK',
      metadata: {
        ...taskDetails.metadata,
        updated_by: 'sdk',
        updated_at: new Date().toISOString(),
      },
    });
    console.log('Updated task:', updatedTask.name);

    // List projects
    console.log('\n📁 Listing projects...');
    const projects = await client.listProjects();
    console.log(`Found ${projects.length} projects:`);
    projects.forEach(p => console.log(`  - ${p.name} (${p.status})`));

    // Get server metrics
    console.log('\n📊 Getting server metrics...');
    const metrics = await client.getMetrics();
    console.log('Server metrics:', metrics);

    // Cancel task
    console.log('\n❌ Cancelling task...');
    const cancelledTask = await client.cancelTask(task.id, 'Test cancellation via SDK');
    console.log('Cancelled task:', cancelledTask.name);

    console.log('\n✅ SDK example completed successfully!');

  } catch (error) {
    console.error('❌ Error:', error);
    
    if (error instanceof Error) {
      console.error('Error name:', error.name);
      console.error('Error message:', error.message);
    }
  }
}

// Run the example
main().catch(console.error);