// NovaDesign - Schedule Module

// Calendar and task scheduling functionality

class ScheduleManager {
    constructor() {
        this.currentDate = new Date();
        this.selectedDate = new Date();
        this.tasks = [];
        this.events = [];
        this.view = 'month'; // month, week, day
    }

    // Load tasks and events from storage
    loadData() {
        const savedTasks = localStorage.getItem('scheduleTasks');
        const savedEvents = localStorage.getItem('scheduleEvents');
        
        if (savedTasks) {
            this.tasks = JSON.parse(savedTasks);
        }
        
        if (savedEvents) {
            this.events = JSON.parse(savedEvents);
        }
    }

    // Save tasks and events to storage
    saveData() {
        localStorage.setItem('scheduleTasks', JSON.stringify(this.tasks));
        localStorage.setItem('scheduleEvents', JSON.stringify(this.events));
    }

    // Add new task
    addTask(taskData) {
        const task = {
            id: Date.now(),
            title: taskData.title,
            description: taskData.description || '',
            date: taskData.date,
            startTime: taskData.startTime || '09:00',
            endTime: taskData.endTime || '10:00',
            priority: taskData.priority || 'medium',
            category: taskData.category || 'general',
            projectId: taskData.projectId || null,
            completed: false,
            reminder: taskData.reminder || 15, // minutes before
            createdAt: new Date().toISOString()
        };
        
        this.tasks.push(task);
        this.saveData();
        
        if (window.addActivity) {
            window.addActivity(`Nuova attività programmata: ${task.title}`);
        }
        
        return task;
    }

    // Update task
    updateTask(taskId, updateData) {
        const task = this.tasks.find(t => t.id === taskId);
        if (!task) return null;
        
        Object.keys(updateData).forEach(key => {
            if (updateData[key] !== undefined) {
                task[key] = updateData[key];
            }
        });
        
        task.updatedAt = new Date().toISOString();
        this.saveData();
        
        return task;
    }

    // Delete task
    deleteTask(taskId) {
        const index = this.tasks.findIndex(t => t.id === taskId);
        if (index === -1) return false;
        
        this.tasks.splice(index, 1);
        this.saveData();
        return true;
    }

    // Complete task
    completeTask(taskId) {
        const task = this.tasks.find(t => t.id === taskId);
        if (!task) return false;
        
        task.completed = !task.completed;
        task.completedAt = task.completed ? new Date().toISOString() : null;
        this.saveData();
        
        if (window.addActivity) {
            const action = task.completed ? 'completata' : 'riaperta';
            window.addActivity(`Attività ${action}: ${task.title}`);
        }
        
        return true;
    }

    // Get tasks for specific date
    getTasksForDate(date) {
        const dateStr = this.formatDate(date);
        return this.tasks.filter(task => task.date === dateStr);
    }

    // Get tasks for date range
    getTasksInRange(startDate, endDate) {
        const start = new Date(startDate);
        const end = new Date(endDate);
        
        return this.tasks.filter(task => {
            const taskDate = new Date(task.date);
            return taskDate >= start && taskDate <= end;
        });
    }

    // Get overdue tasks
    getOverdueTasks() {
        const today = this.formatDate(new Date());
        return this.tasks.filter(task => 
            !task.completed && 
            task.date < today
        );
    }

    // Get upcoming tasks (next 7 days)
    getUpcomingTasks() {
        const today = new Date();
        const nextWeek = new Date(today);
        nextWeek.setDate(today.getDate() + 7);
        
        return this.getTasksInRange(today, nextWeek)
            .filter(task => !task.completed)
            .sort((a, b) => {
                if (a.date !== b.date) {
                    return new Date(a.date) - new Date(b.date);
                }
                return a.startTime.localeCompare(b.startTime);
            });
    }

    // Get tasks by priority
    getTasksByPriority(priority) {
        return this.tasks.filter(task => task.priority === priority);
    }

    // Format date as YYYY-MM-DD
    formatDate(date) {
        return date.toISOString().split('T')[0];
    }

    // Get month name
    getMonthName(monthIndex) {
        const months = [
            'Gennaio', 'Febbraio', 'Marzo', 'Aprile', 'Maggio', 'Giugno',
            'Luglio', 'Agosto', 'Settembre', 'Ottobre', 'Novembre', 'Dicembre'
        ];
        return months[monthIndex];
    }

    // Get day name
    getDayName(dayIndex) {
        const days = [
            'Domenica', 'Lunedì', 'Martedì', 'Mercoledì', 'Giovedì', 'Venerdì', 'Sabato'
        ];
        return days[dayIndex];
    }

    // Navigate calendar
    navigateCalendar(direction) {
        if (direction === 'prev') {
            this.currentDate.setMonth(this.currentDate.getMonth() - 1);
        } else if (direction === 'next') {
            this.currentDate.setMonth(this.currentDate.getMonth() + 1);
        }
        
        updateCalendar();
    }

    // Generate calendar days for current month
    generateCalendarDays() {
        const year = this.currentDate.getFullYear();
        const month = this.currentDate.getMonth();
        
        // First day of the month
        const firstDay = new Date(year, month, 1);
        // Last day of the month
        const lastDay = new Date(year, month + 1, 0);
        
        // Days from previous month to fill the week
        const startDate = new Date(firstDay);
        startDate.setDate(startDate.getDate() - firstDay.getDay());
        
        const days = [];
        const current = new Date(startDate);
        
        // Generate 42 days (6 weeks)
        for (let i = 0; i < 42; i++) {
            const isCurrentMonth = current.getMonth() === month;
            const isToday = this.formatDate(current) === this.formatDate(new Date());
            const tasks = this.getTasksForDate(current);
            
            days.push({
                date: new Date(current),
                dayNumber: current.getDate(),
                isCurrentMonth: isCurrentMonth,
                isToday: isToday,
                tasks: tasks,
                hasEvents: tasks.length > 0
            });
            
            current.setDate(current.getDate() + 1);
        }
        
        return days;
    }
}

// Initialize schedule manager
const scheduleManager = new ScheduleManager();

// DOM manipulation functions
function updateCalendar() {
    const calendarGrid = document.getElementById('calendar-grid');
    const currentMonthElement = document.getElementById('current-month');
    
    if (!calendarGrid || !currentMonthElement) return;
    
    // Update month header
    const monthName = scheduleManager.getMonthName(scheduleManager.currentDate.getMonth());
    const year = scheduleManager.currentDate.getFullYear();
    currentMonthElement.textContent = `${monthName} ${year}`;
    
    // Clear calendar
    calendarGrid.innerHTML = '';
    
    // Add day headers
    const dayHeaders = ['Dom', 'Lun', 'Mar', 'Mer', 'Gio', 'Ven', 'Sab'];
    dayHeaders.forEach(day => {
        const dayHeader = document.createElement('div');
        dayHeader.className = 'calendar-day-header';
        dayHeader.textContent = day;
        calendarGrid.appendChild(dayHeader);
    });
    
    // Generate and add calendar days
    const days = scheduleManager.generateCalendarDays();
    days.forEach(dayData => {
        const dayElement = createCalendarDay(dayData);
        calendarGrid.appendChild(dayElement);
    });
}

function createCalendarDay(dayData) {
    const dayElement = document.createElement('div');
    dayElement.className = 'calendar-day';
    
    if (!dayData.isCurrentMonth) {
        dayElement.classList.add('other-month');
    }
    
    if (dayData.isToday) {
        dayElement.classList.add('today');
    }
    
    if (dayData.hasEvents) {
        dayElement.classList.add('has-events');
    }
    
    // Add click handler
    dayElement.addEventListener('click', () => {
        selectDate(dayData.date);
    });
    
    const dayNumber = document.createElement('div');
    dayNumber.className = 'day-number';
    dayNumber.textContent = dayData.dayNumber;
    dayElement.appendChild(dayNumber);
    
    // Add task indicators
    if (dayData.tasks.length > 0) {
        const tasksIndicator = document.createElement('div');
        tasksIndicator.className = 'tasks-indicator';
        
        const completedTasks = dayData.tasks.filter(t => t.completed).length;
        const totalTasks = dayData.tasks.length;
        
        tasksIndicator.innerHTML = `<small>${completedTasks}/${totalTasks}</small>`;
        dayElement.appendChild(tasksIndicator);
        
        // Add priority indicators
        const highPriorityTasks = dayData.tasks.filter(t => t.priority === 'high' && !t.completed);
        if (highPriorityTasks.length > 0) {
            const priorityDot = document.createElement('div');
            priorityDot.className = 'priority-indicator high';
            dayElement.appendChild(priorityDot);
        }
    }
    
    return dayElement;
}

function selectDate(date) {
    scheduleManager.selectedDate = new Date(date);
    updateDailyTasks();
    
    // Update visual selection
    document.querySelectorAll('.calendar-day').forEach(day => {
        day.classList.remove('selected');
    });
    
    // Add selection to clicked day
    event.target.closest('.calendar-day').classList.add('selected');
}

function updateDailyTasks() {
    const dailyTasksContainer = document.getElementById('daily-tasks');
    if (!dailyTasksContainer) return;
    
    const tasks = scheduleManager.getTasksForDate(scheduleManager.selectedDate);
    
    dailyTasksContainer.innerHTML = '';
    
    if (tasks.length === 0) {
        dailyTasksContainer.innerHTML = '<p>Nessuna attività per questo giorno.</p>';
        return;
    }
    
    // Sort tasks by time
    tasks.sort((a, b) => a.startTime.localeCompare(b.startTime));
    
    tasks.forEach(task => {
        const taskElement = createTaskElement(task);
        dailyTasksContainer.appendChild(taskElement);
    });
}

function createTaskElement(task) {
    const taskElement = document.createElement('div');
    taskElement.className = `task-item priority-${task.priority}`;
    
    if (task.completed) {
        taskElement.classList.add('completed');
    }
    
    taskElement.innerHTML = `
        <div class="task-header">
            <div class="task-checkbox">
                <input type="checkbox" ${task.completed ? 'checked' : ''} 
                       onchange="toggleTaskCompletion(${task.id})">
            </div>
            <div class="task-info">
                <div class="task-title">${task.title}</div>
                <div class="task-time">${task.startTime} - ${task.endTime}</div>
                ${task.description ? `<div class="task-description">${task.description}</div>` : ''}
            </div>
            <div class="task-actions">
                <button class="btn-icon" onclick="editTask(${task.id})" title="Modifica">
                    <i class="fas fa-edit"></i>
                </button>
                <button class="btn-icon" onclick="deleteTask(${task.id})" title="Elimina">
                    <i class="fas fa-trash"></i>
                </button>
            </div>
        </div>
        ${task.projectId ? `<div class="task-project">Progetto: ${getProjectName(task.projectId)}</div>` : ''}
    `;
    
    return taskElement;
}

function getProjectName(projectId) {
    if (window.projectManager) {
        const project = window.projectManager.getProject(parseInt(projectId));
        return project ? project.name : 'Progetto sconosciuto';
    }
    return 'Progetto';
}

// Task management functions
function toggleTaskCompletion(taskId) {
    scheduleManager.completeTask(taskId);
    updateDailyTasks();
    updateCalendar();
    
    if (window.updateDashboard) {
        window.updateDashboard();
    }
}

function editTask(taskId) {
    const task = scheduleManager.tasks.find(t => t.id === taskId);
    if (!task) return;
    
    // Show task edit modal (implementation would go here)
    console.log('Edit task:', task);
}

function deleteTask(taskId) {
    if (confirm('Sei sicuro di voler eliminare questa attività?')) {
        scheduleManager.deleteTask(taskId);
        updateDailyTasks();
        updateCalendar();
        
        if (window.updateDashboard) {
            window.updateDashboard();
        }
    }
}

function addNewTask() {
    // Show add task modal (implementation would go here)
    const title = prompt('Titolo attività:');
    if (!title) return;
    
    const taskData = {
        title: title,
        date: scheduleManager.formatDate(scheduleManager.selectedDate),
        startTime: '09:00',
        endTime: '10:00',
        priority: 'medium'
    };
    
    scheduleManager.addTask(taskData);
    updateDailyTasks();
    updateCalendar();
    
    if (window.updateDashboard) {
        window.updateDashboard();
    }
}

// Calendar navigation
function navigateCalendar(direction) {
    scheduleManager.navigateCalendar(direction);
}

// Initialize calendar when DOM is loaded
document.addEventListener('DOMContentLoaded', function() {
    scheduleManager.loadData();
    
    // Bind navigation events
    const prevButton = document.getElementById('prev-month');
    const nextButton = document.getElementById('next-month');
    const addTaskButton = document.getElementById('add-task-btn');
    
    if (prevButton) {
        prevButton.addEventListener('click', () => navigateCalendar('prev'));
    }
    
    if (nextButton) {
        nextButton.addEventListener('click', () => navigateCalendar('next'));
    }
    
    if (addTaskButton) {
        addTaskButton.addEventListener('click', addNewTask);
    }
    
    // Initialize calendar view
    updateCalendar();
    updateDailyTasks();
    
    // Select today by default
    scheduleManager.selectedDate = new Date();
});

// Add some sample tasks for demonstration
function addSampleTasks() {
    const today = new Date();
    const tomorrow = new Date(today);
    tomorrow.setDate(today.getDate() + 1);
    
    const sampleTasks = [
        {
            title: 'Sopralluogo cantiere',
            description: 'Controllo avanzamento lavori',
            date: scheduleManager.formatDate(today),
            startTime: '09:00',
            endTime: '11:00',
            priority: 'high',
            category: 'inspection'
        },
        {
            title: 'Riunione con cliente',
            description: 'Presentazione preventivo',
            date: scheduleManager.formatDate(tomorrow),
            startTime: '14:00',
            endTime: '15:30',
            priority: 'medium',
            category: 'meeting'
        }
    ];
    
    sampleTasks.forEach(task => scheduleManager.addTask(task));
}

// Export for global use
window.scheduleManager = scheduleManager;
window.updateCalendar = updateCalendar;
window.updateDailyTasks = updateDailyTasks;
window.toggleTaskCompletion = toggleTaskCompletion;
window.editTask = editTask;
window.deleteTask = deleteTask;
window.addNewTask = addNewTask;
window.navigateCalendar = navigateCalendar;
window.addSampleTasks = addSampleTasks;