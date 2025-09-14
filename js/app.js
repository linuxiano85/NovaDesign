// NovaDesign - Main Application Script

// Global application state
const app = {
    currentUser: {
        name: 'Professionista',
        category: null,
        language: 'it'
    },
    data: {
        projects: [],
        contacts: [],
        estimates: [],
        tasks: []
    }
};

// Language translations
const translations = {
    it: {
        welcome: 'Benvenuto in NovaDesign',
        selectCategory: 'Seleziona la tua categoria professionale',
        dashboard: 'Dashboard',
        projects: 'Progetti',
        calculator: 'Calcolatore',
        estimates: 'Preventivi',
        schedule: 'Pianificazione',
        contacts: 'Contatti',
        activeProjects: 'Progetti Attivi',
        totalValue: 'Valore Totale',
        pendingTasks: 'Attività Pendenti',
        recentActivity: 'Attività Recente',
        newProject: 'Nuovo Progetto',
        newContact: 'Nuovo Contatto',
        save: 'Salva',
        cancel: 'Annulla',
        edit: 'Modifica',
        delete: 'Elimina',
        view: 'Visualizza'
    },
    en: {
        welcome: 'Welcome to NovaDesign',
        selectCategory: 'Select your professional category',
        dashboard: 'Dashboard',
        projects: 'Projects',
        calculator: 'Calculator',
        estimates: 'Estimates',
        schedule: 'Schedule',
        contacts: 'Contacts',
        activeProjects: 'Active Projects',
        totalValue: 'Total Value',
        pendingTasks: 'Pending Tasks',
        recentActivity: 'Recent Activity',
        newProject: 'New Project',
        newContact: 'New Contact',
        save: 'Save',
        cancel: 'Cancel',
        edit: 'Edit',
        delete: 'Delete',
        view: 'View'
    }
};

// DOM Content Loaded
document.addEventListener('DOMContentLoaded', function() {
    initializeApp();
    bindEvents();
    loadData();
    updateDashboard();
});

// Initialize Application
function initializeApp() {
    // Set up navigation
    setupNavigation();
    
    // Load user preferences
    loadUserPreferences();
    
    // Initialize components
    initializeModals();
    
    console.log('NovaDesign initialized successfully');
}

// Setup Navigation
function setupNavigation() {
    const navLinks = document.querySelectorAll('.nav a[data-section]');
    
    navLinks.forEach(link => {
        link.addEventListener('click', function(e) {
            e.preventDefault();
            const sectionId = this.getAttribute('data-section');
            showSection(sectionId);
            
            // Update active nav link
            navLinks.forEach(l => l.classList.remove('active'));
            this.classList.add('active');
        });
    });
}

// Show Section
function showSection(sectionId) {
    // Hide all sections
    const sections = document.querySelectorAll('.section');
    sections.forEach(section => section.classList.remove('active'));
    
    // Show target section
    const targetSection = document.getElementById(sectionId);
    if (targetSection) {
        targetSection.classList.add('active');
        
        // Update section-specific content
        updateSectionContent(sectionId);
    }
}

// Update Section Content
function updateSectionContent(sectionId) {
    switch(sectionId) {
        case 'dashboard':
            updateDashboard();
            break;
        case 'projects':
            updateProjectsList();
            break;
        case 'contacts':
            updateContactsList();
            break;
        case 'schedule':
            updateCalendar();
            break;
        case 'estimates':
            updateEstimatesList();
            break;
    }
}

// Bind Events
function bindEvents() {
    // Category selection
    const categoryCards = document.querySelectorAll('.category-card');
    categoryCards.forEach(card => {
        card.addEventListener('click', function() {
            selectCategory(this.getAttribute('data-category'));
        });
    });
    
    // Language selection
    const languageSelect = document.getElementById('language-select');
    languageSelect.addEventListener('change', function() {
        changeLanguage(this.value);
    });
    
    // Modal events
    bindModalEvents();
    
    // Form events
    bindFormEvents();
}

// Select Category
function selectCategory(category) {
    // Remove previous selection
    document.querySelectorAll('.category-card').forEach(card => {
        card.classList.remove('selected');
    });
    
    // Add selection to clicked card
    document.querySelector(`[data-category="${category}"]`).classList.add('selected');
    
    // Update app state
    app.currentUser.category = category;
    
    // Save preference
    localStorage.setItem('userCategory', category);
    
    // Add activity
    addActivity(`Categoria professionale selezionata: ${getCategoryName(category)}`);
    
    // Update dashboard
    updateDashboard();
}

// Get Category Name
function getCategoryName(category) {
    const categoryNames = {
        architect: 'Architetto',
        engineer: 'Ingegnere',
        contractor: 'Impresa',
        electrician: 'Elettricista',
        plumber: 'Idraulico',
        carpenter: 'Carpentiere'
    };
    return categoryNames[category] || category;
}

// Change Language
function changeLanguage(language) {
    app.currentUser.language = language;
    localStorage.setItem('userLanguage', language);
    
    // Update UI text
    updateUILanguage();
    
    addActivity(`Lingua cambiata in: ${language === 'it' ? 'Italiano' : 'English'}`);
}

// Update UI Language
function updateUILanguage() {
    const lang = app.currentUser.language;
    const t = translations[lang];
    
    // Update navigation
    document.querySelector('[data-section="dashboard"]').textContent = t.dashboard;
    document.querySelector('[data-section="projects"]').textContent = t.projects;
    document.querySelector('[data-section="calculator"]').textContent = t.calculator;
    document.querySelector('[data-section="estimates"]').textContent = t.estimates;
    document.querySelector('[data-section="schedule"]').textContent = t.schedule;
    document.querySelector('[data-section="contacts"]').textContent = t.contacts;
    
    // Update other elements as needed
}

// Load User Preferences
function loadUserPreferences() {
    const savedCategory = localStorage.getItem('userCategory');
    const savedLanguage = localStorage.getItem('userLanguage');
    
    if (savedCategory) {
        app.currentUser.category = savedCategory;
        document.querySelector(`[data-category="${savedCategory}"]`)?.classList.add('selected');
    }
    
    if (savedLanguage) {
        app.currentUser.language = savedLanguage;
        document.getElementById('language-select').value = savedLanguage;
        updateUILanguage();
    }
}

// Load Data
function loadData() {
    // Load from localStorage
    const savedProjects = localStorage.getItem('projects');
    const savedContacts = localStorage.getItem('contacts');
    const savedEstimates = localStorage.getItem('estimates');
    const savedTasks = localStorage.getItem('tasks');
    
    if (savedProjects) {
        app.data.projects = JSON.parse(savedProjects);
    }
    
    if (savedContacts) {
        app.data.contacts = JSON.parse(savedContacts);
    }
    
    if (savedEstimates) {
        app.data.estimates = JSON.parse(savedEstimates);
    }
    
    if (savedTasks) {
        app.data.tasks = JSON.parse(savedTasks);
    }
}

// Save Data
function saveData() {
    localStorage.setItem('projects', JSON.stringify(app.data.projects));
    localStorage.setItem('contacts', JSON.stringify(app.data.contacts));
    localStorage.setItem('estimates', JSON.stringify(app.data.estimates));
    localStorage.setItem('tasks', JSON.stringify(app.data.tasks));
}

// Update Dashboard
function updateDashboard() {
    // Update stats
    document.getElementById('active-projects').textContent = app.data.projects.filter(p => p.status === 'active').length;
    
    const totalValue = app.data.projects.reduce((sum, project) => sum + (project.budget || 0), 0);
    document.getElementById('total-value').textContent = `€${totalValue.toLocaleString()}`;
    
    document.getElementById('pending-tasks').textContent = app.data.tasks.filter(t => !t.completed).length;
    document.getElementById('total-contacts').textContent = app.data.contacts.length;
}

// Add Activity
function addActivity(message) {
    const activityList = document.getElementById('activity-list');
    const activityItem = document.createElement('div');
    activityItem.className = 'activity-item';
    activityItem.innerHTML = `
        <i class="fas fa-info-circle"></i>
        <span>${message}</span>
        <span class="activity-time">${new Date().toLocaleTimeString()}</span>
    `;
    
    // Add to top of list
    activityList.insertBefore(activityItem, activityList.firstChild);
    
    // Keep only last 10 activities
    while (activityList.children.length > 10) {
        activityList.removeChild(activityList.lastChild);
    }
}

// Initialize Modals
function initializeModals() {
    const modals = document.querySelectorAll('.modal');
    
    modals.forEach(modal => {
        // Close button
        const closeBtn = modal.querySelector('.close');
        closeBtn.addEventListener('click', () => closeModal(modal));
        
        // Click outside to close
        modal.addEventListener('click', (e) => {
            if (e.target === modal) {
                closeModal(modal);
            }
        });
    });
}

// Show Modal
function showModal(modalId) {
    const modal = document.getElementById(modalId);
    if (modal) {
        modal.classList.add('show');
        modal.style.display = 'flex';
    }
}

// Close Modal
function closeModal(modal) {
    modal.classList.remove('show');
    modal.style.display = 'none';
    
    // Clear form if exists
    const form = modal.querySelector('form');
    if (form) {
        form.reset();
    }
}

// Bind Modal Events
function bindModalEvents() {
    // Add Project Modal
    document.getElementById('add-project-btn').addEventListener('click', () => {
        showModal('project-modal');
    });
    
    document.getElementById('cancel-project').addEventListener('click', () => {
        closeModal(document.getElementById('project-modal'));
    });
    
    // Add Contact Modal
    document.getElementById('add-contact-btn').addEventListener('click', () => {
        showModal('contact-modal');
    });
    
    document.getElementById('cancel-contact').addEventListener('click', () => {
        closeModal(document.getElementById('contact-modal'));
    });
}

// Bind Form Events
function bindFormEvents() {
    // Project Form
    document.getElementById('project-form').addEventListener('submit', handleProjectSubmit);
    
    // Contact Form
    document.getElementById('contact-form').addEventListener('submit', handleContactSubmit);
}

// Handle Project Submit
function handleProjectSubmit(e) {
    e.preventDefault();
    
    const formData = new FormData(e.target);
    const project = {
        id: Date.now(),
        name: document.getElementById('project-name').value,
        client: document.getElementById('project-client').value,
        budget: parseFloat(document.getElementById('project-budget').value) || 0,
        startDate: document.getElementById('project-start').value,
        endDate: document.getElementById('project-end').value,
        description: document.getElementById('project-description').value,
        status: 'active',
        progress: 0,
        createdAt: new Date().toISOString()
    };
    
    app.data.projects.push(project);
    saveData();
    
    closeModal(document.getElementById('project-modal'));
    updateProjectsList();
    updateDashboard();
    
    addActivity(`Nuovo progetto creato: ${project.name}`);
}

// Handle Contact Submit
function handleContactSubmit(e) {
    e.preventDefault();
    
    const contact = {
        id: Date.now(),
        name: document.getElementById('contact-name').value,
        company: document.getElementById('contact-company').value,
        type: document.getElementById('contact-type').value,
        phone: document.getElementById('contact-phone').value,
        email: document.getElementById('contact-email').value,
        address: document.getElementById('contact-address').value,
        createdAt: new Date().toISOString()
    };
    
    app.data.contacts.push(contact);
    saveData();
    
    closeModal(document.getElementById('contact-modal'));
    updateContactsList();
    updateDashboard();
    
    addActivity(`Nuovo contatto aggiunto: ${contact.name}`);
}

// Update Projects List
function updateProjectsList() {
    const projectsList = document.getElementById('projects-list');
    projectsList.innerHTML = '';
    
    if (app.data.projects.length === 0) {
        projectsList.innerHTML = '<p>Nessun progetto trovato. Crea il tuo primo progetto!</p>';
        return;
    }
    
    app.data.projects.forEach(project => {
        const projectCard = createProjectCard(project);
        projectsList.appendChild(projectCard);
    });
}

// Create Project Card
function createProjectCard(project) {
    const card = document.createElement('div');
    card.className = 'project-card';
    card.innerHTML = `
        <div class="project-header">
            <div class="project-title">${project.name}</div>
            <div class="project-status status-${project.status}">${getStatusText(project.status)}</div>
        </div>
        <div class="project-info">
            <p><strong>Cliente:</strong> ${project.client}</p>
            <p><strong>Budget:</strong> €${project.budget.toLocaleString()}</p>
            <p><strong>Inizio:</strong> ${project.startDate ? new Date(project.startDate).toLocaleDateString() : 'Non specificato'}</p>
            <p><strong>Fine prevista:</strong> ${project.endDate ? new Date(project.endDate).toLocaleDateString() : 'Non specificata'}</p>
        </div>
        <div class="project-progress">
            <div class="progress-bar" style="width: ${project.progress}%"></div>
        </div>
        <div class="project-actions">
            <button class="btn btn-small btn-secondary" onclick="editProject(${project.id})">Modifica</button>
            <button class="btn btn-small btn-primary" onclick="viewProject(${project.id})">Visualizza</button>
        </div>
    `;
    return card;
}

// Get Status Text
function getStatusText(status) {
    const statusTexts = {
        active: 'Attivo',
        completed: 'Completato',
        pending: 'In attesa'
    };
    return statusTexts[status] || status;
}

// Update Contacts List
function updateContactsList() {
    const contactsList = document.getElementById('contacts-list');
    contactsList.innerHTML = '';
    
    if (app.data.contacts.length === 0) {
        contactsList.innerHTML = '<p>Nessun contatto trovato. Aggiungi il tuo primo contatto!</p>';
        return;
    }
    
    app.data.contacts.forEach(contact => {
        const contactCard = createContactCard(contact);
        contactsList.appendChild(contactCard);
    });
}

// Create Contact Card
function createContactCard(contact) {
    const card = document.createElement('div');
    card.className = 'contact-card';
    card.innerHTML = `
        <div class="contact-header">
            <div class="contact-avatar">${contact.name.charAt(0).toUpperCase()}</div>
            <div class="contact-info">
                <h3>${contact.name}</h3>
                <div class="contact-company">${contact.company || 'Indipendente'}</div>
            </div>
            <div class="contact-type type-${contact.type}">${getContactTypeText(contact.type)}</div>
        </div>
        <div class="contact-details">
            ${contact.phone ? `<div class="contact-detail"><i class="fas fa-phone"></i> ${contact.phone}</div>` : ''}
            ${contact.email ? `<div class="contact-detail"><i class="fas fa-envelope"></i> ${contact.email}</div>` : ''}
            ${contact.address ? `<div class="contact-detail"><i class="fas fa-map-marker-alt"></i> ${contact.address}</div>` : ''}
        </div>
    `;
    return card;
}

// Get Contact Type Text
function getContactTypeText(type) {
    const typeTexts = {
        client: 'Cliente',
        supplier: 'Fornitore',
        collaborator: 'Collaboratore'
    };
    return typeTexts[type] || type;
}

// Update Estimates List
function updateEstimatesList() {
    const estimatesList = document.getElementById('estimates-list');
    estimatesList.innerHTML = '';
    
    if (app.data.estimates.length === 0) {
        estimatesList.innerHTML = '<p>Nessun preventivo trovato. Crea il tuo primo preventivo!</p>';
        return;
    }
    
    // Implementation for estimates list
}

// Utility Functions
function editProject(projectId) {
    console.log('Edit project:', projectId);
    // Implementation for editing project
}

function viewProject(projectId) {
    console.log('View project:', projectId);
    // Implementation for viewing project details
}

// Export for use in other modules
window.app = app;
window.addActivity = addActivity;
window.saveData = saveData;