// NovaDesign - Projects Management Module

// Project class definition
class Project {
    constructor(data) {
        this.id = data.id || Date.now();
        this.name = data.name || '';
        this.client = data.client || '';
        this.budget = data.budget || 0;
        this.startDate = data.startDate || null;
        this.endDate = data.endDate || null;
        this.description = data.description || '';
        this.status = data.status || 'pending';
        this.progress = data.progress || 0;
        this.category = data.category || window.app?.currentUser?.category || 'general';
        this.tasks = data.tasks || [];
        this.documents = data.documents || [];
        this.expenses = data.expenses || [];
        this.notes = data.notes || [];
        this.createdAt = data.createdAt || new Date().toISOString();
        this.updatedAt = data.updatedAt || new Date().toISOString();
    }

    // Update project progress based on completed tasks
    updateProgress() {
        if (this.tasks.length === 0) {
            this.progress = 0;
            return;
        }
        
        const completedTasks = this.tasks.filter(task => task.completed).length;
        this.progress = Math.round((completedTasks / this.tasks.length) * 100);
        this.updatedAt = new Date().toISOString();
        
        // Auto-complete project if all tasks are done
        if (this.progress === 100 && this.status === 'active') {
            this.status = 'completed';
        }
    }

    // Add task to project
    addTask(taskData) {
        const task = {
            id: Date.now(),
            title: taskData.title,
            description: taskData.description || '',
            priority: taskData.priority || 'medium',
            deadline: taskData.deadline || null,
            assignee: taskData.assignee || '',
            completed: false,
            createdAt: new Date().toISOString()
        };
        
        this.tasks.push(task);
        this.updateProgress();
        return task;
    }

    // Add expense to project
    addExpense(expenseData) {
        const expense = {
            id: Date.now(),
            description: expenseData.description,
            amount: parseFloat(expenseData.amount) || 0,
            category: expenseData.category || 'materials',
            date: expenseData.date || new Date().toISOString().split('T')[0],
            receipt: expenseData.receipt || null,
            createdAt: new Date().toISOString()
        };
        
        this.expenses.push(expense);
        this.updatedAt = new Date().toISOString();
        return expense;
    }

    // Get total expenses
    getTotalExpenses() {
        return this.expenses.reduce((total, expense) => total + expense.amount, 0);
    }

    // Get remaining budget
    getRemainingBudget() {
        return this.budget - this.getTotalExpenses();
    }

    // Get project duration in days
    getDuration() {
        if (!this.startDate || !this.endDate) return null;
        
        const start = new Date(this.startDate);
        const end = new Date(this.endDate);
        const diffTime = Math.abs(end - start);
        return Math.ceil(diffTime / (1000 * 60 * 60 * 24));
    }

    // Check if project is overdue
    isOverdue() {
        if (!this.endDate || this.status === 'completed') return false;
        
        const today = new Date();
        const endDate = new Date(this.endDate);
        return today > endDate;
    }

    // Export project data
    export() {
        return {
            id: this.id,
            name: this.name,
            client: this.client,
            budget: this.budget,
            startDate: this.startDate,
            endDate: this.endDate,
            description: this.description,
            status: this.status,
            progress: this.progress,
            category: this.category,
            tasks: this.tasks,
            documents: this.documents,
            expenses: this.expenses,
            notes: this.notes,
            createdAt: this.createdAt,
            updatedAt: this.updatedAt
        };
    }
}

// Project Manager class
class ProjectManager {
    constructor() {
        this.projects = [];
        this.currentProject = null;
        this.filters = {
            status: 'all',
            category: 'all',
            client: 'all'
        };
    }

    // Load projects from storage
    loadProjects() {
        const savedProjects = localStorage.getItem('projects');
        if (savedProjects) {
            const projectsData = JSON.parse(savedProjects);
            this.projects = projectsData.map(data => new Project(data));
        }
    }

    // Save projects to storage
    saveProjects() {
        const projectsData = this.projects.map(project => project.export());
        localStorage.setItem('projects', JSON.stringify(projectsData));
        
        // Update app data
        if (window.app) {
            window.app.data.projects = projectsData;
        }
    }

    // Add new project
    addProject(projectData) {
        const project = new Project(projectData);
        this.projects.push(project);
        this.saveProjects();
        
        if (window.addActivity) {
            window.addActivity(`Nuovo progetto creato: ${project.name}`);
        }
        
        return project;
    }

    // Update existing project
    updateProject(projectId, updateData) {
        const project = this.getProject(projectId);
        if (!project) return null;
        
        Object.keys(updateData).forEach(key => {
            if (updateData[key] !== undefined) {
                project[key] = updateData[key];
            }
        });
        
        project.updatedAt = new Date().toISOString();
        this.saveProjects();
        
        if (window.addActivity) {
            window.addActivity(`Progetto aggiornato: ${project.name}`);
        }
        
        return project;
    }

    // Delete project
    deleteProject(projectId) {
        const index = this.projects.findIndex(p => p.id === projectId);
        if (index === -1) return false;
        
        const project = this.projects[index];
        this.projects.splice(index, 1);
        this.saveProjects();
        
        if (window.addActivity) {
            window.addActivity(`Progetto eliminato: ${project.name}`);
        }
        
        return true;
    }

    // Get project by ID
    getProject(projectId) {
        return this.projects.find(p => p.id === projectId);
    }

    // Get filtered projects
    getFilteredProjects() {
        return this.projects.filter(project => {
            if (this.filters.status !== 'all' && project.status !== this.filters.status) {
                return false;
            }
            
            if (this.filters.category !== 'all' && project.category !== this.filters.category) {
                return false;
            }
            
            if (this.filters.client !== 'all' && project.client !== this.filters.client) {
                return false;
            }
            
            return true;
        });
    }

    // Get projects by status
    getProjectsByStatus(status) {
        return this.projects.filter(p => p.status === status);
    }

    // Get overdue projects
    getOverdueProjects() {
        return this.projects.filter(p => p.isOverdue());
    }

    // Get projects statistics
    getStatistics() {
        const total = this.projects.length;
        const active = this.getProjectsByStatus('active').length;
        const completed = this.getProjectsByStatus('completed').length;
        const pending = this.getProjectsByStatus('pending').length;
        const overdue = this.getOverdueProjects().length;
        
        const totalBudget = this.projects.reduce((sum, p) => sum + p.budget, 0);
        const totalExpenses = this.projects.reduce((sum, p) => sum + p.getTotalExpenses(), 0);
        
        return {
            total,
            active,
            completed,
            pending,
            overdue,
            totalBudget,
            totalExpenses,
            remainingBudget: totalBudget - totalExpenses
        };
    }

    // Search projects
    searchProjects(query) {
        const searchTerm = query.toLowerCase();
        return this.projects.filter(project => 
            project.name.toLowerCase().includes(searchTerm) ||
            project.client.toLowerCase().includes(searchTerm) ||
            project.description.toLowerCase().includes(searchTerm)
        );
    }

    // Generate project report
    generateReport(projectId) {
        const project = this.getProject(projectId);
        if (!project) return null;
        
        const report = {
            project: project.export(),
            summary: {
                duration: project.getDuration(),
                totalExpenses: project.getTotalExpenses(),
                remainingBudget: project.getRemainingBudget(),
                tasksCompleted: project.tasks.filter(t => t.completed).length,
                totalTasks: project.tasks.length,
                isOverdue: project.isOverdue()
            },
            timeline: this.getProjectTimeline(projectId),
            expenses: project.expenses,
            generatedAt: new Date().toISOString()
        };
        
        return report;
    }

    // Get project timeline
    getProjectTimeline(projectId) {
        const project = this.getProject(projectId);
        if (!project) return [];
        
        const timeline = [];
        
        // Add project milestones
        timeline.push({
            date: project.createdAt,
            type: 'project_created',
            description: 'Progetto creato'
        });
        
        if (project.startDate) {
            timeline.push({
                date: project.startDate,
                type: 'project_started',
                description: 'Progetto iniziato'
            });
        }
        
        // Add task completions
        project.tasks.filter(t => t.completed).forEach(task => {
            timeline.push({
                date: task.completedAt || task.createdAt,
                type: 'task_completed',
                description: `Attività completata: ${task.title}`
            });
        });
        
        // Add expenses
        project.expenses.forEach(expense => {
            timeline.push({
                date: expense.date,
                type: 'expense_added',
                description: `Spesa: ${expense.description} - €${expense.amount}`
            });
        });
        
        if (project.status === 'completed') {
            timeline.push({
                date: project.updatedAt,
                type: 'project_completed',
                description: 'Progetto completato'
            });
        }
        
        // Sort by date
        return timeline.sort((a, b) => new Date(a.date) - new Date(b.date));
    }
}

// Initialize project manager
const projectManager = new ProjectManager();

// DOM manipulation functions
function updateProjectsList() {
    const projectsList = document.getElementById('projects-list');
    if (!projectsList) return;
    
    projectsList.innerHTML = '';
    
    const projects = projectManager.getFilteredProjects();
    
    if (projects.length === 0) {
        projectsList.innerHTML = `
            <div class="empty-state">
                <i class="fas fa-project-diagram"></i>
                <h3>Nessun progetto trovato</h3>
                <p>Crea il tuo primo progetto per iniziare!</p>
            </div>
        `;
        return;
    }
    
    projects.forEach(project => {
        const projectCard = createProjectCard(project);
        projectsList.appendChild(projectCard);
    });
}

function createProjectCard(project) {
    const card = document.createElement('div');
    card.className = 'project-card';
    
    const statusClass = `status-${project.status}`;
    const isOverdue = project.isOverdue();
    const remainingBudget = project.getRemainingBudget();
    
    card.innerHTML = `
        <div class="project-header">
            <div class="project-title">${project.name}</div>
            <div class="project-status ${statusClass}">
                ${getStatusText(project.status)}
                ${isOverdue ? '<i class="fas fa-exclamation-triangle" title="In ritardo"></i>' : ''}
            </div>
        </div>
        
        <div class="project-info">
            <p><strong>Cliente:</strong> ${project.client}</p>
            <p><strong>Budget:</strong> €${project.budget.toLocaleString()}</p>
            <p><strong>Speso:</strong> €${project.getTotalExpenses().toLocaleString()}</p>
            <p><strong>Rimanente:</strong> <span class="${remainingBudget < 0 ? 'text-danger' : 'text-success'}">
                €${remainingBudget.toLocaleString()}
            </span></p>
            ${project.startDate ? `<p><strong>Inizio:</strong> ${new Date(project.startDate).toLocaleDateString()}</p>` : ''}
            ${project.endDate ? `<p><strong>Fine:</strong> ${new Date(project.endDate).toLocaleDateString()}</p>` : ''}
        </div>
        
        <div class="project-progress">
            <div class="progress-label">Progresso: ${project.progress}%</div>
            <div class="progress-bar-container">
                <div class="progress-bar" style="width: ${project.progress}%"></div>
            </div>
        </div>
        
        <div class="project-stats">
            <span><i class="fas fa-tasks"></i> ${project.tasks.filter(t => t.completed).length}/${project.tasks.length} attività</span>
            <span><i class="fas fa-euro-sign"></i> ${project.expenses.length} spese</span>
            <span><i class="fas fa-file-alt"></i> ${project.documents.length} documenti</span>
        </div>
        
        <div class="project-actions">
            <button class="btn btn-small btn-secondary" onclick="editProject(${project.id})">
                <i class="fas fa-edit"></i> Modifica
            </button>
            <button class="btn btn-small btn-primary" onclick="viewProject(${project.id})">
                <i class="fas fa-eye"></i> Visualizza
            </button>
            <button class="btn btn-small btn-danger" onclick="deleteProject(${project.id})">
                <i class="fas fa-trash"></i> Elimina
            </button>
        </div>
    `;
    
    return card;
}

function getStatusText(status) {
    const statusTexts = {
        pending: 'In attesa',
        active: 'Attivo',
        completed: 'Completato',
        cancelled: 'Annullato'
    };
    return statusTexts[status] || status;
}

// Project action functions
function editProject(projectId) {
    const project = projectManager.getProject(projectId);
    if (!project) return;
    
    // Populate form with project data
    document.getElementById('project-name').value = project.name;
    document.getElementById('project-client').value = project.client;
    document.getElementById('project-budget').value = project.budget;
    document.getElementById('project-start').value = project.startDate || '';
    document.getElementById('project-end').value = project.endDate || '';
    document.getElementById('project-description').value = project.description;
    
    // Change form submit behavior for editing
    const form = document.getElementById('project-form');
    form.onsubmit = function(e) {
        e.preventDefault();
        updateProjectFromForm(projectId);
    };
    
    // Show modal
    showModal('project-modal');
}

function updateProjectFromForm(projectId) {
    const updateData = {
        name: document.getElementById('project-name').value,
        client: document.getElementById('project-client').value,
        budget: parseFloat(document.getElementById('project-budget').value) || 0,
        startDate: document.getElementById('project-start').value || null,
        endDate: document.getElementById('project-end').value || null,
        description: document.getElementById('project-description').value
    };
    
    projectManager.updateProject(projectId, updateData);
    closeModal(document.getElementById('project-modal'));
    updateProjectsList();
    
    if (window.updateDashboard) {
        window.updateDashboard();
    }
    
    // Reset form behavior
    const form = document.getElementById('project-form');
    form.onsubmit = function(e) {
        e.preventDefault();
        handleProjectSubmit(e);
    };
}

function viewProject(projectId) {
    const project = projectManager.getProject(projectId);
    if (!project) return;
    
    // Create detailed view modal or navigate to project details page
    console.log('Viewing project:', project);
    
    // For now, show an alert with project details
    const report = projectManager.generateReport(projectId);
    const details = `
Progetto: ${project.name}
Cliente: ${project.client}
Stato: ${getStatusText(project.status)}
Progresso: ${project.progress}%
Budget: €${project.budget.toLocaleString()}
Speso: €${project.getTotalExpenses().toLocaleString()}
Rimanente: €${project.getRemainingBudget().toLocaleString()}
Attività: ${project.tasks.filter(t => t.completed).length}/${project.tasks.length}
    `;
    
    alert(details);
}

function deleteProject(projectId) {
    const project = projectManager.getProject(projectId);
    if (!project) return;
    
    if (confirm(`Sei sicuro di voler eliminare il progetto "${project.name}"?`)) {
        projectManager.deleteProject(projectId);
        updateProjectsList();
        
        if (window.updateDashboard) {
            window.updateDashboard();
        }
    }
}

// Enhanced project form submission
function handleProjectSubmit(e) {
    e.preventDefault();
    
    const projectData = {
        name: document.getElementById('project-name').value,
        client: document.getElementById('project-client').value,
        budget: parseFloat(document.getElementById('project-budget').value) || 0,
        startDate: document.getElementById('project-start').value || null,
        endDate: document.getElementById('project-end').value || null,
        description: document.getElementById('project-description').value,
        status: 'pending'
    };
    
    projectManager.addProject(projectData);
    closeModal(document.getElementById('project-modal'));
    updateProjectsList();
    
    if (window.updateDashboard) {
        window.updateDashboard();
    }
}

// Initialize when DOM is loaded
document.addEventListener('DOMContentLoaded', function() {
    projectManager.loadProjects();
    
    // Override the original project form handler
    const projectForm = document.getElementById('project-form');
    if (projectForm) {
        projectForm.removeEventListener('submit', window.handleProjectSubmit);
        projectForm.addEventListener('submit', handleProjectSubmit);
    }
});

// Export for global use
window.projectManager = projectManager;
window.updateProjectsList = updateProjectsList;
window.editProject = editProject;
window.viewProject = viewProject;
window.deleteProject = deleteProject;
window.handleProjectSubmit = handleProjectSubmit;