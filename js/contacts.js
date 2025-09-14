// NovaDesign - Contacts Management Module

// Contact management for construction professionals

class ContactsManager {
    constructor() {
        this.contacts = [];
        this.filters = {
            type: '',
            search: ''
        };
    }

    // Load contacts from storage
    loadContacts() {
        const savedContacts = localStorage.getItem('contacts');
        if (savedContacts) {
            this.contacts = JSON.parse(savedContacts);
        }
    }

    // Save contacts to storage
    saveContacts() {
        localStorage.setItem('contacts', JSON.stringify(this.contacts));
        
        // Update app data
        if (window.app) {
            window.app.data.contacts = this.contacts;
        }
    }

    // Add new contact
    addContact(contactData) {
        const contact = {
            id: Date.now(),
            name: contactData.name,
            company: contactData.company || '',
            type: contactData.type, // client, supplier, collaborator
            phone: contactData.phone || '',
            email: contactData.email || '',
            address: contactData.address || '',
            notes: contactData.notes || '',
            tags: contactData.tags || [],
            projects: [], // Associated project IDs
            interactions: [], // History of interactions
            rating: contactData.rating || 0,
            isActive: true,
            createdAt: new Date().toISOString(),
            updatedAt: new Date().toISOString()
        };

        this.contacts.push(contact);
        this.saveContacts();

        if (window.addActivity) {
            window.addActivity(`Nuovo contatto aggiunto: ${contact.name}`);
        }

        return contact;
    }

    // Update existing contact
    updateContact(contactId, updateData) {
        const contact = this.getContact(contactId);
        if (!contact) return null;

        Object.keys(updateData).forEach(key => {
            if (updateData[key] !== undefined) {
                contact[key] = updateData[key];
            }
        });

        contact.updatedAt = new Date().toISOString();
        this.saveContacts();

        if (window.addActivity) {
            window.addActivity(`Contatto aggiornato: ${contact.name}`);
        }

        return contact;
    }

    // Delete contact
    deleteContact(contactId) {
        const index = this.contacts.findIndex(c => c.id === contactId);
        if (index === -1) return false;

        const contact = this.contacts[index];
        this.contacts.splice(index, 1);
        this.saveContacts();

        if (window.addActivity) {
            window.addActivity(`Contatto eliminato: ${contact.name}`);
        }

        return true;
    }

    // Get contact by ID
    getContact(contactId) {
        return this.contacts.find(c => c.id === contactId);
    }

    // Get contacts by type
    getContactsByType(type) {
        return this.contacts.filter(c => c.type === type && c.isActive);
    }

    // Search contacts
    searchContacts(query) {
        const searchTerm = query.toLowerCase();
        return this.contacts.filter(contact => 
            contact.isActive && (
                contact.name.toLowerCase().includes(searchTerm) ||
                contact.company.toLowerCase().includes(searchTerm) ||
                contact.email.toLowerCase().includes(searchTerm) ||
                contact.phone.includes(searchTerm)
            )
        );
    }

    // Get filtered contacts
    getFilteredContacts() {
        let filtered = this.contacts.filter(c => c.isActive);

        if (this.filters.type) {
            filtered = filtered.filter(c => c.type === this.filters.type);
        }

        if (this.filters.search) {
            const searchTerm = this.filters.search.toLowerCase();
            filtered = filtered.filter(contact => 
                contact.name.toLowerCase().includes(searchTerm) ||
                contact.company.toLowerCase().includes(searchTerm) ||
                contact.email.toLowerCase().includes(searchTerm) ||
                contact.phone.includes(searchTerm)
            );
        }

        return filtered.sort((a, b) => a.name.localeCompare(b.name));
    }

    // Add interaction to contact
    addInteraction(contactId, interactionData) {
        const contact = this.getContact(contactId);
        if (!contact) return null;

        const interaction = {
            id: Date.now(),
            type: interactionData.type, // call, email, meeting, project
            description: interactionData.description,
            date: interactionData.date || new Date().toISOString(),
            outcome: interactionData.outcome || '',
            followUpDate: interactionData.followUpDate || null,
            createdAt: new Date().toISOString()
        };

        contact.interactions.push(interaction);
        contact.updatedAt = new Date().toISOString();
        this.saveContacts();

        return interaction;
    }

    // Associate contact with project
    associateWithProject(contactId, projectId) {
        const contact = this.getContact(contactId);
        if (!contact) return false;

        if (!contact.projects.includes(projectId)) {
            contact.projects.push(projectId);
            contact.updatedAt = new Date().toISOString();
            this.saveContacts();
        }

        return true;
    }

    // Get contact statistics
    getStatistics() {
        const total = this.contacts.filter(c => c.isActive).length;
        const clients = this.getContactsByType('client').length;
        const suppliers = this.getContactsByType('supplier').length;
        const collaborators = this.getContactsByType('collaborator').length;

        return {
            total,
            clients,
            suppliers,
            collaborators,
            inactive: this.contacts.filter(c => !c.isActive).length
        };
    }

    // Export contacts to CSV
    exportToCSV() {
        const headers = ['Nome', 'Azienda', 'Tipo', 'Telefono', 'Email', 'Indirizzo'];
        const csvContent = [
            headers.join(','),
            ...this.contacts.map(contact => [
                contact.name,
                contact.company,
                this.getContactTypeText(contact.type),
                contact.phone,
                contact.email,
                contact.address.replace(/\n/g, ' ')
            ].map(field => `"${field}"`).join(','))
        ].join('\n');

        return csvContent;
    }

    // Import contacts from CSV
    importFromCSV(csvData) {
        const lines = csvData.split('\n');
        const headers = lines[0].split(',');
        
        let imported = 0;
        for (let i = 1; i < lines.length; i++) {
            const values = lines[i].split(',');
            if (values.length >= 3) {
                const contactData = {
                    name: values[0]?.replace(/"/g, '') || '',
                    company: values[1]?.replace(/"/g, '') || '',
                    type: this.getContactTypeFromText(values[2]?.replace(/"/g, '') || ''),
                    phone: values[3]?.replace(/"/g, '') || '',
                    email: values[4]?.replace(/"/g, '') || '',
                    address: values[5]?.replace(/"/g, '') || ''
                };

                if (contactData.name && contactData.type) {
                    this.addContact(contactData);
                    imported++;
                }
            }
        }

        return imported;
    }

    // Helper methods
    getContactTypeText(type) {
        const typeTexts = {
            client: 'Cliente',
            supplier: 'Fornitore',
            collaborator: 'Collaboratore'
        };
        return typeTexts[type] || type;
    }

    getContactTypeFromText(text) {
        const textToType = {
            'Cliente': 'client',
            'Fornitore': 'supplier',
            'Collaboratore': 'collaborator'
        };
        return textToType[text] || 'client';
    }

    // Get recent contacts (last 30 days)
    getRecentContacts() {
        const thirtyDaysAgo = new Date();
        thirtyDaysAgo.setDate(thirtyDaysAgo.getDate() - 30);

        return this.contacts
            .filter(c => c.isActive && new Date(c.updatedAt) > thirtyDaysAgo)
            .sort((a, b) => new Date(b.updatedAt) - new Date(a.updatedAt));
    }

    // Get contacts with upcoming follow-ups
    getFollowUpContacts() {
        const today = new Date();
        const nextWeek = new Date(today);
        nextWeek.setDate(today.getDate() + 7);

        return this.contacts.filter(contact => {
            return contact.interactions.some(interaction => 
                interaction.followUpDate && 
                new Date(interaction.followUpDate) >= today &&
                new Date(interaction.followUpDate) <= nextWeek
            );
        });
    }
}

// Initialize contacts manager
const contactsManager = new ContactsManager();

// DOM manipulation functions
function updateContactsList() {
    const contactsList = document.getElementById('contacts-list');
    if (!contactsList) return;

    contactsList.innerHTML = '';

    const contacts = contactsManager.getFilteredContacts();

    if (contacts.length === 0) {
        contactsList.innerHTML = `
            <div class="empty-state">
                <i class="fas fa-address-book"></i>
                <h3>Nessun contatto trovato</h3>
                <p>Aggiungi il tuo primo contatto per iniziare!</p>
            </div>
        `;
        return;
    }

    contacts.forEach(contact => {
        const contactCard = createContactCard(contact);
        contactsList.appendChild(contactCard);
    });
}

function createContactCard(contact) {
    const card = document.createElement('div');
    card.className = 'contact-card';

    const initials = contact.name.split(' ').map(n => n[0]).join('').toUpperCase().slice(0, 2);
    const typeClass = `type-${contact.type}`;
    const rating = '★'.repeat(contact.rating) + '☆'.repeat(5 - contact.rating);

    card.innerHTML = `
        <div class="contact-header">
            <div class="contact-avatar">${initials}</div>
            <div class="contact-info">
                <h3>${contact.name}</h3>
                <div class="contact-company">${contact.company || 'Indipendente'}</div>
                ${contact.rating > 0 ? `<div class="contact-rating">${rating}</div>` : ''}
            </div>
            <div class="contact-type ${typeClass}">
                ${contactsManager.getContactTypeText(contact.type)}
            </div>
        </div>
        
        <div class="contact-details">
            ${contact.phone ? `
                <div class="contact-detail">
                    <i class="fas fa-phone"></i>
                    <a href="tel:${contact.phone}">${contact.phone}</a>
                </div>
            ` : ''}
            ${contact.email ? `
                <div class="contact-detail">
                    <i class="fas fa-envelope"></i>
                    <a href="mailto:${contact.email}">${contact.email}</a>
                </div>
            ` : ''}
            ${contact.address ? `
                <div class="contact-detail">
                    <i class="fas fa-map-marker-alt"></i>
                    <span>${contact.address}</span>
                </div>
            ` : ''}
        </div>
        
        ${contact.projects.length > 0 ? `
            <div class="contact-projects">
                <small><i class="fas fa-project-diagram"></i> ${contact.projects.length} progetti associati</small>
            </div>
        ` : ''}
        
        <div class="contact-actions">
            <button class="btn btn-small btn-secondary" onclick="editContact(${contact.id})">
                <i class="fas fa-edit"></i> Modifica
            </button>
            <button class="btn btn-small btn-primary" onclick="viewContact(${contact.id})">
                <i class="fas fa-eye"></i> Visualizza
            </button>
            <button class="btn btn-small btn-info" onclick="addInteraction(${contact.id})">
                <i class="fas fa-comment"></i> Interazione
            </button>
        </div>
    `;

    return card;
}

// Contact action functions
function editContact(contactId) {
    const contact = contactsManager.getContact(contactId);
    if (!contact) return;

    // Populate form with contact data
    document.getElementById('contact-name').value = contact.name;
    document.getElementById('contact-company').value = contact.company;
    document.getElementById('contact-type').value = contact.type;
    document.getElementById('contact-phone').value = contact.phone;
    document.getElementById('contact-email').value = contact.email;
    document.getElementById('contact-address').value = contact.address;

    // Change form submit behavior for editing
    const form = document.getElementById('contact-form');
    form.onsubmit = function(e) {
        e.preventDefault();
        updateContactFromForm(contactId);
    };

    // Show modal
    if (window.showModal) {
        window.showModal('contact-modal');
    }
}

function updateContactFromForm(contactId) {
    const updateData = {
        name: document.getElementById('contact-name').value,
        company: document.getElementById('contact-company').value,
        type: document.getElementById('contact-type').value,
        phone: document.getElementById('contact-phone').value,
        email: document.getElementById('contact-email').value,
        address: document.getElementById('contact-address').value
    };

    contactsManager.updateContact(contactId, updateData);
    
    if (window.closeModal) {
        window.closeModal(document.getElementById('contact-modal'));
    }
    
    updateContactsList();

    if (window.updateDashboard) {
        window.updateDashboard();
    }

    // Reset form behavior
    const form = document.getElementById('contact-form');
    form.onsubmit = function(e) {
        e.preventDefault();
        handleContactSubmit(e);
    };
}

function viewContact(contactId) {
    const contact = contactsManager.getContact(contactId);
    if (!contact) return;

    // Create detailed view
    const details = `
Contatto: ${contact.name}
Azienda: ${contact.company || 'Indipendente'}
Tipo: ${contactsManager.getContactTypeText(contact.type)}
Telefono: ${contact.phone || 'Non specificato'}
Email: ${contact.email || 'Non specificata'}
Indirizzo: ${contact.address || 'Non specificato'}
Progetti associati: ${contact.projects.length}
Interazioni registrate: ${contact.interactions.length}
Creato: ${new Date(contact.createdAt).toLocaleDateString()}
Ultimo aggiornamento: ${new Date(contact.updatedAt).toLocaleDateString()}
    `;

    alert(details);
}

function addInteraction(contactId) {
    const contact = contactsManager.getContact(contactId);
    if (!contact) return;

    const description = prompt(`Aggiungi interazione per ${contact.name}:`);
    if (!description) return;

    const interactionData = {
        type: 'note',
        description: description,
        date: new Date().toISOString()
    };

    contactsManager.addInteraction(contactId, interactionData);
    
    if (window.addActivity) {
        window.addActivity(`Interazione aggiunta per ${contact.name}`);
    }
}

function deleteContact(contactId) {
    const contact = contactsManager.getContact(contactId);
    if (!contact) return;

    if (confirm(`Sei sicuro di voler eliminare il contatto "${contact.name}"?`)) {
        contactsManager.deleteContact(contactId);
        updateContactsList();

        if (window.updateDashboard) {
            window.updateDashboard();
        }
    }
}

// Enhanced contact form submission
function handleContactSubmit(e) {
    e.preventDefault();

    const contactData = {
        name: document.getElementById('contact-name').value,
        company: document.getElementById('contact-company').value,
        type: document.getElementById('contact-type').value,
        phone: document.getElementById('contact-phone').value,
        email: document.getElementById('contact-email').value,
        address: document.getElementById('contact-address').value
    };

    contactsManager.addContact(contactData);
    
    if (window.closeModal) {
        window.closeModal(document.getElementById('contact-modal'));
    }
    
    updateContactsList();

    if (window.updateDashboard) {
        window.updateDashboard();
    }
}

// Filter functions
function applyContactFilters() {
    const typeFilter = document.getElementById('contact-type-filter');
    const searchFilter = document.getElementById('contact-search');

    if (typeFilter) {
        contactsManager.filters.type = typeFilter.value;
    }

    if (searchFilter) {
        contactsManager.filters.search = searchFilter.value;
    }

    updateContactsList();
}

// Export/Import functions
function exportContacts() {
    const csvContent = contactsManager.exportToCSV();
    const blob = new Blob([csvContent], { type: 'text/csv' });
    const url = window.URL.createObjectURL(blob);
    
    const a = document.createElement('a');
    a.href = url;
    a.download = 'contatti_novadesign.csv';
    a.click();
    
    window.URL.revokeObjectURL(url);
    
    if (window.addActivity) {
        window.addActivity('Contatti esportati in CSV');
    }
}

function importContacts() {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = '.csv';
    
    input.onchange = function(e) {
        const file = e.target.files[0];
        if (!file) return;
        
        const reader = new FileReader();
        reader.onload = function(e) {
            const csvData = e.target.result;
            const imported = contactsManager.importFromCSV(csvData);
            
            alert(`${imported} contatti importati con successo!`);
            updateContactsList();
            
            if (window.updateDashboard) {
                window.updateDashboard();
            }
            
            if (window.addActivity) {
                window.addActivity(`${imported} contatti importati da CSV`);
            }
        };
        reader.readAsText(file);
    };
    
    input.click();
}

// Initialize when DOM is loaded
document.addEventListener('DOMContentLoaded', function() {
    contactsManager.loadContacts();

    // Bind filter events
    const typeFilter = document.getElementById('contact-type-filter');
    const searchFilter = document.getElementById('contact-search');

    if (typeFilter) {
        typeFilter.addEventListener('change', applyContactFilters);
    }

    if (searchFilter) {
        searchFilter.addEventListener('input', applyContactFilters);
    }

    // Override the original contact form handler
    const contactForm = document.getElementById('contact-form');
    if (contactForm) {
        contactForm.removeEventListener('submit', window.handleContactSubmit);
        contactForm.addEventListener('submit', handleContactSubmit);
    }
});

// Export for global use
window.contactsManager = contactsManager;
window.updateContactsList = updateContactsList;
window.editContact = editContact;
window.viewContact = viewContact;
window.addInteraction = addInteraction;
window.deleteContact = deleteContact;
window.handleContactSubmit = handleContactSubmit;
window.exportContacts = exportContacts;
window.importContacts = importContacts;