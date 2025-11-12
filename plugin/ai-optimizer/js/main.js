/**
 * Pixly AI Optimizer - Main Application
 * Handles UI interactions and application flow
 */

// Initialize application
let aiClient;
let optimizer;
let currentLanguage = 'en';

document.addEventListener('DOMContentLoaded', async () => {
    console.log('Pixly AI Optimizer starting...');
    
    // Initialize AI client
    aiClient = new PixlyAIClient();
    optimizer = new PixlyOptimizer(aiClient);
    
    // Initialize i18n
    await initI18n();
    
    // Setup event listeners
    setupEventListeners();
    
    // Check AI service health
    await checkAIService();
    
    console.log('Pixly AI Optimizer ready!');
});

/**
 * Initialize internationalization
 */
async function initI18n() {
    // Detect language (could be from Eagle API or browser)
    currentLanguage = navigator.language.startsWith('zh') ? 'zh_CN' : 'en';
    
    // Load translations
    try {
        const response = await fetch(`_locales/${currentLanguage}.json`);
        if (response.ok) {
            const translations = await response.json();
            applyTranslations(translations);
        }
    } catch (error) {
        console.warn('Failed to load translations:', error);
    }
}

/**
 * Apply translations to DOM
 */
function applyTranslations(translations) {
    document.querySelectorAll('[data-i18n]').forEach(element => {
        const key = element.getAttribute('data-i18n');
        const text = getNestedValue(translations, key);
        if (text) {
            element.textContent = text;
        }
    });
}

/**
 * Get nested object value by dot notation
 */
function getNestedValue(obj, path) {
    return path.split('.').reduce((current, key) => current?.[key], obj);
}

/**
 * Setup event listeners
 */
function setupEventListeners() {
    // Drop zone
    const dropZone = document.getElementById('dropZone');
    const fileInput = document.getElementById('fileInput');
    
    dropZone.addEventListener('click', () => fileInput.click());
    
    dropZone.addEventListener('dragover', (e) => {
        e.preventDefault();
        dropZone.classList.add('drag-over');
    });
    
    dropZone.addEventListener('dragleave', () => {
        dropZone.classList.remove('drag-over');
    });
    
    dropZone.addEventListener('drop', (e) => {
        e.preventDefault();
        dropZone.classList.remove('drag-over');
        handleFiles(e.dataTransfer.files);
    });
    
    fileInput.addEventListener('change', (e) => {
        handleFiles(e.target.files);
    });
    
    // Mode cards
    document.querySelectorAll('.mode-card').forEach(card => {
        card.addEventListener('click', () => {
            document.querySelectorAll('.mode-card').forEach(c => c.classList.remove('active'));
            card.classList.add('active');
            optimizer.setMode(card.dataset.mode);
        });
    });
    
    // Action buttons
    document.getElementById('cancelBtn')?.addEventListener('click', resetApp);
    document.getElementById('optimizeBtn')?.addEventListener('click', startOptimization);
    document.getElementById('doneBtn')?.addEventListener('click', resetApp);
}

/**
 * Check AI service availability
 */
async function checkAIService() {
    try {
        const health = await aiClient.checkHealth();
        console.log('AI Service:', health);
    } catch (error) {
        console.error('AI Service unavailable:', error);
        showError('AI service is not available. Please ensure the Python AI service is running on port 50052.');
    }
}

/**
 * Handle file selection
 */
function handleFiles(files) {
    if (!files || files.length === 0) return;
    
    const fileList = optimizer.addFiles(files);
    displayFiles(fileList);
    
    // Show mode selection and actions
    document.getElementById('fileList').style.display = 'block';
    document.getElementById('modeSection').style.display = 'block';
    document.getElementById('actionsSection').style.display = 'block';
}

/**
 * Display selected files
 */
function displayFiles(files) {
    const container = document.getElementById('filesContainer');
    container.innerHTML = '';
    
    files.forEach(file => {
        const fileItem = document.createElement('div');
        fileItem.className = 'file-item';
        fileItem.innerHTML = `
            <div class="file-info">
                <span class="file-icon">${getFileIcon(file.type)}</span>
                <div class="file-details">
                    <div class="file-name">${file.name}</div>
                    <div class="file-size">${PixlyOptimizer.formatSize(file.size)}</div>
                </div>
            </div>
        `;
        container.appendChild(fileItem);
    });
}

/**
 * Get file icon by type
 */
function getFileIcon(type) {
    const icons = {
        'image': '🖼️',
        'video': '🎬',
        'audio': '🎵',
        'unknown': '📄'
    };
    return icons[type] || icons.unknown;
}

/**
 * Start optimization process
 */
async function startOptimization() {
    // Hide actions and mode selection
    document.getElementById('actionsSection').style.display = 'none';
    document.getElementById('modeSection').style.display = 'none';
    
    // Show progress
    document.getElementById('progressSection').style.display = 'block';
    
    try {
        // Optimize files
        await optimizer.optimize((completed, total) => {
            updateProgress(completed, total);
        });
        
        // Show results
        showResults();
        
    } catch (error) {
        console.error('Optimization error:', error);
        showError('Optimization failed: ' + error.message);
    }
}

/**
 * Update progress bar
 */
function updateProgress(completed, total) {
    const percentage = (completed / total) * 100;
    document.getElementById('progressFill').style.width = `${percentage}%`;
    document.getElementById('progressText').textContent = `${completed} / ${total}`;
}

/**
 * Show optimization results
 */
function showResults() {
    // Hide progress
    document.getElementById('progressSection').style.display = 'none';
    
    // Show results section
    document.getElementById('resultsSection').style.display = 'block';
    
    // Display statistics
    const stats = optimizer.getStats();
    displayStats(stats);
    
    // Display individual results
    displayResultsList(optimizer.results);
}

/**
 * Display optimization statistics
 */
function displayStats(stats) {
    const statsGrid = document.getElementById('statsGrid');
    statsGrid.innerHTML = `
        <div class="stat-card">
            <div class="stat-value">${stats.successful}</div>
            <div class="stat-label">Successful</div>
        </div>
        <div class="stat-card">
            <div class="stat-value">${stats.failed}</div>
            <div class="stat-label">Failed</div>
        </div>
        <div class="stat-card">
            <div class="stat-value">${PixlyOptimizer.formatSize(stats.savedSize)}</div>
            <div class="stat-label">Space Saved</div>
        </div>
        <div class="stat-card success">
            <div class="stat-value">${stats.savedRatio}%</div>
            <div class="stat-label">Reduction</div>
        </div>
    `;
}

/**
 * Display results list
 */
function displayResultsList(results) {
    const resultsList = document.getElementById('resultsList');
    resultsList.innerHTML = '';
    
    results.forEach(result => {
        const resultItem = document.createElement('div');
        resultItem.className = `result-item ${result.status}`;
        
        if (result.status === 'success') {
            const ratio = ((result.result.originalSize - result.result.optimizedSize) / result.result.originalSize * 100).toFixed(1);
            resultItem.innerHTML = `
                <div class="result-icon">✅</div>
                <div class="result-info">
                    <div class="result-name">${result.file.name}</div>
                    <div class="result-details">
                        ${PixlyOptimizer.formatSize(result.result.originalSize)} → 
                        ${PixlyOptimizer.formatSize(result.result.optimizedSize)} 
                        (${ratio}% reduction)
                    </div>
                </div>
            `;
        } else {
            resultItem.innerHTML = `
                <div class="result-icon">❌</div>
                <div class="result-info">
                    <div class="result-name">${result.file.name}</div>
                    <div class="result-error">${result.error}</div>
                </div>
            `;
        }
        
        resultsList.appendChild(resultItem);
    });
}

/**
 * Show error message
 */
function showError(message) {
    // Could implement a toast notification here
    alert(message);
}

/**
 * Reset application to initial state
 */
function resetApp() {
    // Reset optimizer
    optimizer = new PixlyOptimizer(aiClient);
    
    // Hide all sections except drop zone
    document.getElementById('fileList').style.display = 'none';
    document.getElementById('modeSection').style.display = 'none';
    document.getElementById('analysisSection').style.display = 'none';
    document.getElementById('actionsSection').style.display = 'none';
    document.getElementById('progressSection').style.display = 'none';
    document.getElementById('resultsSection').style.display = 'none';
    
    // Clear file input
    document.getElementById('fileInput').value = '';
    
    // Reset mode to balanced
    document.querySelectorAll('.mode-card').forEach(c => c.classList.remove('active'));
    document.querySelector('[data-mode="balanced"]').classList.add('active');
}
