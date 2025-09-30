const { createApp, ref, computed, onMounted, onBeforeUnmount, watch } = Vue;

createApp({
    setup() {
        // Reactive state
        const currentPage = ref('overview');
        const loading = ref(false);
        const connected = ref(true);
        const refreshInterval = ref(null);
        
        // Data
        const tasks = ref([]);
        const workflows = ref([]);
        const projects = ref([]);
        const stats = ref({
            total_tasks: 0,
            active_tasks: 0,
            pending_tasks: 0,
            completed_tasks: 0,
            failed_tasks: 0,
            total_workflows: 0,
            total_projects: 0,
            cpu_usage_percent: 0,
            memory_usage_mb: 0,
            uptime_seconds: 0
        });
        const metrics = ref({});
        
        // Filters
        const taskFilters = ref({
            status: '',
            project: '',
            priority: ''
        });
        
        const dependencyFilters = ref({
            correlationId: '',
            taskId: ''
        });
        
        // Computed properties
        const pageTitle = computed(() => {
            const titles = {
                overview: 'Overview',
                projects: 'Projects',
                tasks: 'Tasks',
                workflows: 'Workflows',
                dependencies: 'Dependencies',
                metrics: 'Metrics',
                console: 'Console'
            };
            return titles[currentPage.value] || currentPage.value;
        });

        const recentTasks = computed(() => {
            return tasks.value
                .sort((a, b) => new Date(b.created_at) - new Date(a.created_at))
                .slice(0, 10);
        });

        const projectNames = computed(() => {
            console.log('Computing projectNames, projects.value:', projects.value);
            if (!projects.value || !Array.isArray(projects.value)) {
                console.log('Projects is not an array:', projects.value);
                return [];
            }
            const names = projects.value.map(project => {
                if (!project || !project.name) {
                    console.log('Invalid project:', project);
                    return null;
                }
                return project.name;
            }).filter(Boolean).sort();
            console.log('Project names:', names);
            return names;
        });

        const developmentPhases = computed(() => {
            const phases = [
                { status: 'Planning', label: 'Planning', icon: 'fas fa-file-alt' },
                { status: 'Implementation', label: 'Implementation', icon: 'fas fa-code' },
                { status: 'TestCreation', label: 'Test Creation', icon: 'fas fa-vial' },
                { status: 'Testing', label: 'Testing', icon: 'fas fa-stethoscope' },
                { status: 'AIReview', label: 'AI Review', icon: 'fas fa-brain' },
                { status: 'Finalized', label: 'Finalized', icon: 'fas fa-check-circle' },
                { status: 'Completed', label: 'Completed', icon: 'fas fa-check-circle' }
            ];

            return phases.map(phase => ({
                ...phase,
                count: tasks.value.filter(task => task.status === phase.status).length
            }));
        });

        const selectedTaskDependencies = ref([]);

        // Methods
        const setPage = (page) => {
            currentPage.value = page;
            if (page === 'overview' || page === 'tasks') {
                startAutoRefresh();
            } else {
                stopAutoRefresh();
            }
        };

        const refreshData = async () => {
            loading.value = true;
            try {
                await Promise.all([
                    loadStats(),
                    loadTasks(),
                    loadWorkflows(),
                    loadProjects(),
                    loadMetrics()
                ]);
            } catch (error) {
                console.error('Error refreshing data:', error);
                showToast('Failed to refresh data', 'error');
            } finally {
                loading.value = false;
            }
        };

        const loadStats = async () => {
            try {
                const data = await window.apiClient.getStats();
                stats.value = data;
            } catch (error) {
                console.error('Error loading stats:', error);
            }
        };

        const loadTasks = async () => {
            try {
                const data = await window.apiClient.listTasks(
                    taskFilters.value.project || null,
                    taskFilters.value.status || null
                );
                tasks.value = Array.isArray(data) ? data : [];
            } catch (error) {
                console.error('Error loading tasks:', error);
                tasks.value = [];
            }
        };

        const loadWorkflows = async () => {
            try {
                const data = await window.apiClient.listWorkflows();
                workflows.value = Array.isArray(data) ? data : [];
            } catch (error) {
                console.error('Error loading workflows:', error);
                workflows.value = [];
            }
        };

        const loadProjects = async () => {
            try {
                console.log('Loading projects...');
                const data = await window.apiClient.listProjects();
                console.log('Projects data received:', data);
                console.log('Projects data type:', typeof data);
                console.log('Projects data is array:', Array.isArray(data));
                if (data && data.length > 0) {
                    console.log('First project:', data[0]);
                    console.log('First project keys:', Object.keys(data[0]));
                }
                projects.value = Array.isArray(data) ? data : [];
                console.log('Projects set to:', projects.value);
                console.log('Projects length:', projects.value.length);
            } catch (error) {
                console.error('Error loading projects:', error);
                projects.value = [];
            }
        };

        const loadMetrics = async () => {
            try {
                const data = await window.apiClient.getMetrics();
                metrics.value = data;
            } catch (error) {
                console.error('Error loading metrics:', error);
                metrics.value = {};
            }
        };

        const loadDependencies = async () => {
            if (!dependencyFilters.value.taskId) {
                selectedTaskDependencies.value = [];
                return;
            }

            try {
                const data = await window.apiClient.getTaskDependencies(dependencyFilters.value.taskId);
                selectedTaskDependencies.value = Array.isArray(data) ? data : [];
            } catch (error) {
                console.error('Error loading dependencies:', error);
                selectedTaskDependencies.value = [];
            }
        };

        // Task management
        const advanceTaskPhase = async (taskId) => {
            try {
                await window.apiClient.advanceTaskPhase(taskId);
                showToast('Task phase advanced successfully', 'success');
                await refreshData();
            } catch (error) {
                console.error('Error advancing task phase:', error);
                showToast('Failed to advance task phase', 'error');
            }
        };

        const retryTask = async (taskId) => {
            try {
                await window.apiClient.retryTask(taskId);
                showToast('Task retry initiated', 'success');
                await refreshData();
            } catch (error) {
                console.error('Error retrying task:', error);
                showToast('Failed to retry task', 'error');
            }
        };

        const cancelTask = async (taskId) => {
            if (!confirm('Are you sure you want to cancel this task?')) {
                return;
            }

            try {
                await window.apiClient.cancelTask(taskId);
                showToast('Task cancelled successfully', 'success');
                await refreshData();
            } catch (error) {
                console.error('Error cancelling task:', error);
                showToast('Failed to cancel task', 'error');
            }
        };

        // Utility functions
        const formatStatus = (status) => {
            const statusMap = {
                'Pending': 'Pending',
                'Running': 'Running',
                'Completed': 'Completed',
                'Failed': 'Failed',
                'Cancelled': 'Cancelled',
                'Planning': 'Planning',
                'Implementation': 'Implementation',
                'TestCreation': 'Test Creation',
                'Testing': 'Testing',
                'AIReview': 'AI Review',
                'Finalized': 'Finalized',
                'WaitingForDependencies': 'Waiting for Dependencies',
                'AnalysisAndDocumentation': 'Analysis & Documentation',
                'InDiscussion': 'In Discussion',
                'InImplementation': 'In Implementation',
                'InReview': 'In Review',
                'InTesting': 'In Testing'
            };
            return statusMap[status] || status;
        };

        const getStatusClass = (status) => {
            const classMap = {
                'Pending': 'pending',
                'Running': 'running',
                'Completed': 'completed',
                'Failed': 'failed',
                'Cancelled': 'cancelled',
                'Planning': 'planning',
                'Implementation': 'implementation',
                'TestCreation': 'testing',
                'Testing': 'testing',
                'AIReview': 'review',
                'Finalized': 'completed',
                'WaitingForDependencies': 'pending',
                'AnalysisAndDocumentation': 'planning',
                'InDiscussion': 'planning',
                'InImplementation': 'implementation',
                'InReview': 'review',
                'InTesting': 'testing'
            };
            return classMap[status] || 'pending';
        };

        const formatPriority = (priority) => {
            const priorityMap = {
                'Low': 'Low',
                'Normal': 'Normal',
                'High': 'High',
                'Critical': 'Critical'
            };
            return priorityMap[priority] || priority;
        };

        const getPriorityClass = (priority) => {
            return priority.toLowerCase();
        };

        const formatWorkflowStatus = (status) => {
            const statusMap = {
                'Pending': 'Pending',
                'Running': 'Running',
                'Completed': 'Completed',
                'Failed': 'Failed',
                'Cancelled': 'Cancelled'
            };
            return statusMap[status] || status;
        };

        const getWorkflowStatusClass = (status) => {
            return status.toLowerCase();
        };

        const formatTime = (timestamp) => {
            if (!timestamp) return 'Unknown';
            try {
                const date = new Date(timestamp);
                return date.toLocaleString();
            } catch (error) {
                return timestamp;
            }
        };

        const formatCondition = (condition) => {
            const conditionMap = {
                'Success': 'Success',
                'Failure': 'Failure',
                'Completion': 'Completion'
            };
            return conditionMap[condition] || condition;
        };

        const getTaskName = (taskId) => {
            const task = tasks.value.find(t => t.id === taskId);
            return task ? task.name : taskId;
        };

        // Modal functions
        const showCreateTaskModal = () => {
            const modalContent = `
                <div class="modal-header">
                    <h2><i class="fas fa-plus"></i> Create New Task</h2>
                    <button class="modal-close" onclick="closeModal()">&times;</button>
                </div>
                <div class="modal-body">
                    <form id="create-task-form">
                        <div class="form-group">
                            <label for="task-name">Task Name</label>
                            <input type="text" id="task-name" class="form-control" placeholder="My Task" required>
                        </div>
                        <div class="form-group">
                            <label for="task-command">Command</label>
                            <input type="text" id="task-command" class="form-control" placeholder="echo 'Hello World'" required>
                        </div>
                        <div class="form-group">
                            <label for="task-project">Project</label>
                            <input type="text" id="task-project" class="form-control" placeholder="my-project" required>
                        </div>
                        <div class="form-group">
                            <label for="task-type">Task Type</label>
                            <select id="task-type" class="form-control" required>
                                <option value="Simple">Simple</option>
                                <option value="Dependent">Dependent</option>
                                <option value="Workflow">Workflow</option>
                                <option value="Scheduled">Scheduled</option>
                            </select>
                        </div>
                        <div class="form-group">
                            <label for="task-priority">Priority</label>
                            <select id="task-priority" class="form-control" required>
                                <option value="Low">Low</option>
                                <option value="Normal" selected>Normal</option>
                                <option value="High">High</option>
                                <option value="Critical">Critical</option>
                            </select>
                        </div>
                        <div class="form-group">
                            <label for="task-description">Description (Optional)</label>
                            <textarea id="task-description" class="form-control" rows="3" placeholder="Describe what this task does..."></textarea>
                        </div>
                    </form>
                </div>
                <div class="modal-footer">
                    <button class="btn btn-secondary" onclick="closeModal()">Cancel</button>
                    <button class="btn btn-primary" onclick="createTask()">
                        <i class="fas fa-plus"></i> Create Task
                    </button>
                </div>
            `;
            showModal(modalContent);
        };

        const showCreateWorkflowModal = () => {
            const modalContent = `
                <div class="modal-header">
                    <h2><i class="fas fa-plus"></i> Create New Workflow</h2>
                    <button class="modal-close" onclick="closeModal()">&times;</button>
                </div>
                <div class="modal-body">
                    <form id="create-workflow-form">
                        <div class="form-group">
                            <label for="workflow-name">Workflow Name</label>
                            <input type="text" id="workflow-name" class="form-control" placeholder="My Workflow" required>
                        </div>
                        <div class="form-group">
                            <label for="workflow-description">Description</label>
                            <textarea id="workflow-description" class="form-control" rows="3" placeholder="Describe what this workflow does..."></textarea>
                        </div>
                    </form>
                </div>
                <div class="modal-footer">
                    <button class="btn btn-secondary" onclick="closeModal()">Cancel</button>
                    <button class="btn btn-primary" onclick="createWorkflow()">
                        <i class="fas fa-plus"></i> Create Workflow
                    </button>
                </div>
            `;
            showModal(modalContent);
        };

        const showTaskMenu = (taskId, event) => {
            event.stopPropagation();
            const task = tasks.value.find(t => t.id === taskId);
            if (!task) return;

            const menuContent = `
                <div class="dropdown-menu" style="position: absolute; top: 100%; right: 0; background: var(--bg-card); border: 1px solid var(--border); border-radius: var(--radius-md); padding: var(--space-2); z-index: 1000;">
                    <button class="dropdown-item" onclick="viewTaskDetails('${taskId}'); hideDropdown()">
                        <i class="fas fa-eye"></i> View Details
                    </button>
                    <button class="dropdown-item" onclick="showTaskDependencies('${taskId}'); hideDropdown()">
                        <i class="fas fa-sitemap"></i> Dependencies
                    </button>
                    ${task.is_in_development ? `
                        <button class="dropdown-item" onclick="advanceTaskPhase('${taskId}'); hideDropdown()">
                            <i class="fas fa-forward"></i> Advance Phase
                        </button>
                    ` : ''}
                    ${task.status === 'Failed' ? `
                        <button class="dropdown-item" onclick="retryTask('${taskId}'); hideDropdown()">
                            <i class="fas fa-redo"></i> Retry
                        </button>
                    ` : ''}
                    <hr style="margin: var(--space-1) 0; border-color: var(--border);">
                    <button class="dropdown-item danger" onclick="cancelTask('${taskId}'); hideDropdown()">
                        <i class="fas fa-times"></i> Cancel
                    </button>
                </div>
            `;
            
            // Remove existing dropdowns
            document.querySelectorAll('.dropdown-menu').forEach(menu => menu.remove());
            
            // Add new dropdown
            const button = event.target.closest('.btn');
            button.style.position = 'relative';
            button.insertAdjacentHTML('afterend', menuContent);
            
            // Close dropdown when clicking outside
            setTimeout(() => {
                document.addEventListener('click', hideDropdown);
            }, 100);
        };

        const hideDropdown = () => {
            document.querySelectorAll('.dropdown-menu').forEach(menu => menu.remove());
            document.removeEventListener('click', hideDropdown);
        };

        const viewTaskDetails = async (taskId) => {
            try {
                const task = await window.apiClient.getTask(taskId);
                const modalContent = `
                    <div class="modal-header">
                        <h2><i class="fas fa-tasks"></i> Task Details</h2>
                        <button class="modal-close" onclick="closeModal()">&times;</button>
                    </div>
                    <div class="modal-body">
                        <div class="task-details-grid">
                            <div class="detail-section">
                                <h3>Basic Information</h3>
                                <div class="detail-list">
                                    <div class="detail-item">
                                        <span class="label">Name:</span>
                                        <span class="value">${task.name}</span>
                                    </div>
                                    <div class="detail-item">
                                        <span class="label">Status:</span>
                                        <span class="value status-${getStatusClass(task.status)}">${formatStatus(task.status)}</span>
                                    </div>
                                    <div class="detail-item">
                                        <span class="label">Project:</span>
                                        <span class="value">${task.project_id ? (projects.value.find(p => p.id === task.project_id)?.name || 'Unknown Project') : 'No Project'}</span>
                                    </div>
                                    <div class="detail-item">
                                        <span class="label">Priority:</span>
                                        <span class="value priority-${getPriorityClass(task.priority)}">${formatPriority(task.priority)}</span>
                                    </div>
                                    <div class="detail-item">
                                        <span class="label">Type:</span>
                                        <span class="value">${task.task_type}</span>
                                    </div>
                                    <div class="detail-item">
                                        <span class="label">Created:</span>
                                        <span class="value">${formatTime(task.created_at)}</span>
                                    </div>
                                    <div class="detail-item">
                                        <span class="label">Updated:</span>
                                        <span class="value">${formatTime(task.updated_at)}</span>
                                    </div>
                                </div>
                            </div>
                            ${task.description ? `
                            <div class="detail-section">
                                <h3>Description</h3>
                                <div class="description-display">
                                    <p>${task.description}</p>
                                </div>
                            </div>
                            ` : ''}
                            ${task.technical_specs ? `
                            <div class="detail-section">
                                <h3>Technical Specifications</h3>
                                <div class="specs-display">
                                    <p>${task.technical_specs}</p>
                                </div>
                            </div>
                            ` : ''}
                            ${task.acceptance_criteria && task.acceptance_criteria.length > 0 ? `
                            <div class="detail-section">
                                <h3>Acceptance Criteria (${task.acceptance_criteria.length})</h3>
                                <div class="criteria-list">
                                    ${task.acceptance_criteria.map(criterion => `
                                        <div class="criterion-item">
                                            <i class="fas fa-check-circle"></i>
                                            <span>${criterion}</span>
                                        </div>
                                    `).join('')}
                                </div>
                            </div>
                            ` : ''}
                            <div class="detail-section">
                                <h3>Command</h3>
                                <div class="command-display">
                                    <code>${task.command}</code>
                                </div>
                            </div>
                            ${task.dependencies && task.dependencies.length > 0 ? `
                                <div class="detail-section">
                                    <h3>Dependencies (${task.dependencies.length})</h3>
                                    <div class="dependencies-list">
                                        ${task.dependencies.map(dep => `
                                            <div class="dependency-item">
                                                <span class="dependency-task">${getTaskName(dep.task_id)}</span>
                                                <span class="dependency-condition">${formatCondition(dep.condition)}</span>
                                                <span class="dependency-required ${dep.required ? 'yes' : 'no'}">${dep.required ? 'Required' : 'Optional'}</span>
                                            </div>
                                        `).join('')}
                                    </div>
                                </div>
                            ` : ''}
                        </div>
                    </div>
                    <div class="modal-footer">
                        <button class="btn btn-secondary" onclick="closeModal()">Close</button>
                    </div>
                `;
                showModal(modalContent);
            } catch (error) {
                showToast('Failed to load task details', 'error');
            }
        };

        const viewWorkflowDetails = async (workflow) => {
            const modalContent = `
                <div class="modal-header">
                    <h2><i class="fas fa-project-diagram"></i> Workflow Details</h2>
                    <button class="modal-close" onclick="closeModal()">&times;</button>
                </div>
                <div class="modal-body">
                    <div class="workflow-details-grid">
                        <div class="detail-section">
                            <h3>Basic Information</h3>
                            <div class="detail-list">
                                <div class="detail-item">
                                    <span class="label">Name:</span>
                                    <span class="value">${workflow.name}</span>
                                </div>
                                <div class="detail-item">
                                    <span class="label">Status:</span>
                                    <span class="value status-${getWorkflowStatusClass(workflow.status)}">${formatWorkflowStatus(workflow.status)}</span>
                                </div>
                                <div class="detail-item">
                                    <span class="label">Tasks:</span>
                                    <span class="value">${workflow.tasks.length}</span>
                                </div>
                                <div class="detail-item">
                                    <span class="label">Created:</span>
                                    <span class="value">${formatTime(workflow.created_at)}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="modal-footer">
                    <button class="btn btn-secondary" onclick="closeModal()">Close</button>
                </div>
            `;
            showModal(modalContent);
        };

        const createTask = async () => {
            const name = document.getElementById('task-name').value.trim();
            const command = document.getElementById('task-command').value.trim();
            const project = document.getElementById('task-project').value.trim();
            const taskType = document.getElementById('task-type').value;
            const priority = document.getElementById('task-priority').value;
            const description = document.getElementById('task-description').value.trim();

            if (!name || !command || !project) {
                showToast('Please fill in all required fields', 'error');
                return;
            }

            try {
                const taskData = {
                    name,
                    command,
                    project,
                    task_type: taskType,
                    priority,
                    description: description || null
                };

                await window.apiClient.createTask(taskData);
                closeModal();
                showToast(`Task "${name}" created successfully`, 'success');
                await refreshData();
            } catch (error) {
                console.error('Error creating task:', error);
                showToast(`Error creating task: ${error.message}`, 'error');
            }
        };

        const createWorkflow = async () => {
            const name = document.getElementById('workflow-name').value.trim();
            const description = document.getElementById('workflow-description').value.trim();

            if (!name) {
                showToast('Workflow name is required', 'error');
                return;
            }

            try {
                const workflowData = {
                    name,
                    description: description || null,
                    tasks: []
                };

                await window.apiClient.createWorkflow(workflowData);
                closeModal();
                showToast(`Workflow "${name}" created successfully`, 'success');
                await refreshData();
            } catch (error) {
                console.error('Error creating workflow:', error);
                showToast(`Error creating workflow: ${error.message}`, 'error');
            }
        };

        // Console functions
        const executeConsoleRequest = async () => {
            const method = document.getElementById('console-method').value;
            const endpoint = document.getElementById('console-endpoint').value;
            const bodyText = document.getElementById('console-body').value.trim();
            
            const statusEl = document.getElementById('response-status');
            const responseEl = document.getElementById('console-response');

            try {
                let responseData;
                
                if (method === 'GET') {
                    responseData = await window.apiClient.request(endpoint);
                } else if (method === 'POST') {
                    const body = bodyText ? JSON.parse(bodyText) : {};
                    responseData = await window.apiClient.request(endpoint, {
                        method: 'POST',
                        body: JSON.stringify(body)
                    });
                } else if (method === 'PUT') {
                    const body = bodyText ? JSON.parse(bodyText) : {};
                    responseData = await window.apiClient.request(endpoint, {
                        method: 'PUT',
                        body: JSON.stringify(body)
                    });
                } else if (method === 'DELETE') {
                    responseData = await window.apiClient.request(endpoint, {
                        method: 'DELETE'
                    });
                }
                
                statusEl.textContent = '200 OK';
                statusEl.className = 'response-status success';
                responseEl.textContent = JSON.stringify(responseData, null, 2);
            } catch (error) {
                statusEl.textContent = 'Error';
                statusEl.className = 'response-status error';
                responseEl.textContent = error.message;
            }
        };

        const clearConsole = () => {
            document.getElementById('console-endpoint').value = '/tasks';
            document.getElementById('console-body').value = '';
            document.getElementById('response-status').textContent = '';
            document.getElementById('console-response').textContent = 'Execute a request to see the response...';
        };

        // Auto refresh
        const startAutoRefresh = () => {
            stopAutoRefresh();
            refreshInterval.value = setInterval(async () => {
                if (currentPage.value === 'overview' || currentPage.value === 'tasks') {
                    await loadStats();
                    await loadTasks();
                }
            }, 5000); // Refresh every 5 seconds
        };

        const stopAutoRefresh = () => {
            if (refreshInterval.value) {
                clearInterval(refreshInterval.value);
                refreshInterval.value = null;
            }
        };

        // Watch for filter changes
        watch(() => dependencyFilters.value.taskId, () => {
            loadDependencies();
        });

        // Global functions for modal actions
        window.advanceTaskPhase = advanceTaskPhase;
        window.retryTask = retryTask;
        window.cancelTask = cancelTask;
        window.viewTaskDetails = viewTaskDetails;
        window.showTaskDependencies = (taskId) => {
            dependencyFilters.value.taskId = taskId;
            setPage('dependencies');
        };
        window.hideDropdown = hideDropdown;

        // Modal functions
        const showModal = (content) => {
            const overlay = document.getElementById('modal-overlay');
            const modal = document.getElementById('modal-content');
            
            modal.innerHTML = content;
            overlay.style.display = 'flex';
            
            // Add event listeners for modal close buttons
            const closeButtons = modal.querySelectorAll('.modal-close');
            closeButtons.forEach(button => {
                button.addEventListener('click', closeModal);
            });
            
            // Close modal when clicking overlay
            overlay.addEventListener('click', (e) => {
                if (e.target === overlay) {
                    closeModal();
                }
            });
        };

        const closeModal = () => {
            const overlay = document.getElementById('modal-overlay');
            overlay.style.display = 'none';
        };

        window.showModal = showModal;
        window.closeModal = closeModal;

        // Toast functions
        const showToast = (message, type = 'info') => {
            const container = document.getElementById('toast-container');
            const toast = document.createElement('div');
            toast.className = `toast toast-${type}`;
            toast.innerHTML = `
                <i class="fas fa-${getToastIcon(type)}"></i>
                <span>${message}</span>
            `;
            
            container.appendChild(toast);
            
            setTimeout(() => toast.classList.add('show'), 100);
            setTimeout(() => {
                toast.classList.remove('show');
                setTimeout(() => container.removeChild(toast), 300);
            }, 3000);
        };

        const getToastIcon = (type) => {
            const icons = {
                success: 'check-circle',
                error: 'exclamation-circle',
                warning: 'exclamation-triangle',
                info: 'info-circle'
            };
            return icons[type] || 'info-circle';
        };

        // Project functions
        const showCreateProjectModal = () => {
            const modalContent = `
                <div class="modal-header">
                    <h3><i class="fas fa-folder-plus"></i> Create Project</h3>
                    <button class="btn-close" onclick="closeModal()">&times;</button>
                </div>
                <div class="modal-body">
                    <form id="create-project-form">
                        <div class="form-group">
                            <label for="project-name">Project Name *</label>
                            <input type="text" id="project-name" name="name" required 
                                   placeholder="Enter project name">
                        </div>
                        <div class="form-group">
                            <label for="project-description">Description</label>
                            <textarea id="project-description" name="description" rows="3"
                                      placeholder="Enter project description"></textarea>
                        </div>
                    </form>
                </div>
                <div class="modal-footer">
                    <button class="btn btn-secondary" onclick="closeModal()">Cancel</button>
                    <button class="btn btn-primary" onclick="createProject()">
                        <i class="fas fa-plus"></i> Create Project
                    </button>
                </div>
            `;
            showModal(modalContent);
        };

        const createProject = async () => {
            const form = document.getElementById('create-project-form');
            const formData = new FormData(form);
            
            const projectData = {
                name: formData.get('name'),
                description: formData.get('description') || null
            };

            try {
                await window.apiClient.createProject(projectData);
                showToast('Project created successfully', 'success');
                closeModal();
                await loadProjects();
            } catch (error) {
                console.error('Error creating project:', error);
                showToast('Failed to create project', 'error');
            }
        };

        const viewProject = (projectId) => {
            if (!projectId) {
                showToast('Invalid project ID', 'error');
                return;
            }
            // Filter tasks by project
            taskFilters.value.project = projectId;
            setPage('tasks');
        };

        const editProject = (projectId) => {
            if (!projectId) {
                showToast('Invalid project ID', 'error');
                return;
            }
            showToast('Edit project functionality coming soon', 'info');
        };

        const formatProjectStatus = (status) => {
            if (!status) return 'Unknown';
            const statusMap = {
                'Planning': 'Planning',
                'Active': 'Active',
                'OnHold': 'On Hold',
                'Completed': 'Completed',
                'Cancelled': 'Cancelled'
            };
            return statusMap[status] || status;
        };

        const getProjectStatusClass = (status) => {
            if (!status) return 'status-default';
            const classMap = {
                'Planning': 'status-planning',
                'Active': 'status-active',
                'OnHold': 'status-warning',
                'Completed': 'status-success',
                'Cancelled': 'status-error'
            };
            return classMap[status] || 'status-default';
        };

        const formatDate = (dateString) => {
            if (!dateString) return 'N/A';
            try {
                const date = new Date(dateString);
                return date.toLocaleDateString('pt-BR', {
                    year: 'numeric',
                    month: 'short',
                    day: 'numeric'
                });
            } catch (error) {
                return 'Invalid Date';
            }
        };

        window.showToast = showToast;
        window.createProject = createProject;

        // Lifecycle
        onMounted(async () => {
            // Test API connection first
            const connection = await window.apiClient.testConnection();
            if (!connection.connected) {
                showToast('API connection failed', 'error');
                connected.value = false;
            } else {
                connected.value = true;
            }
            
            await refreshData();
            startAutoRefresh();
        });

        onBeforeUnmount(() => {
            stopAutoRefresh();
        });

        return {
            currentPage,
            loading,
            connected,
            pageTitle,
            stats,
            metrics,
            tasks,
            workflows,
            recentTasks,
            projects,
            projectNames,
            developmentPhases,
            taskFilters,
            dependencyFilters,
            selectedTaskDependencies,
            setPage,
            refreshData,
            loadTasks,
            loadDependencies,
            advanceTaskPhase,
            retryTask,
            formatStatus,
            getStatusClass,
            formatPriority,
            getPriorityClass,
            formatWorkflowStatus,
            getWorkflowStatusClass,
            formatTime,
            formatCondition,
            getTaskName,
            showCreateTaskModal,
            showCreateWorkflowModal,
            showTaskMenu,
            viewTaskDetails,
            viewWorkflowDetails,
            executeConsoleRequest,
            clearConsole,
            showCreateProjectModal,
            viewProject,
            editProject,
            formatProjectStatus,
            getProjectStatusClass,
            formatDate
        };
    }
}).mount('#app');
