/**
 * 🔍 下拉菜单调试工具
 * 在控制台运行 debugDropdown() 查看状态
 */

window.debugDropdown = function() {
    const container = document.getElementById('convertBtnContainer');
    const button = document.getElementById('convertBtn');
    const dropdown = document.getElementById('convertDropdown');
    
    log.info('=== 🔍 Dropdown Debug Info ===');
    log.info('1. Container:', container);
    log.info('   - classList:', container?.classList.value);
    log.info('   - style.position:', getComputedStyle(container).position);
    
    log.info('2. Button:', button);
    log.info('   - disabled:', button?.disabled);
    log.info('   - offsetWidth:', button?.offsetWidth);
    log.info('   - offsetHeight:', button?.offsetHeight);
    
    log.info('3. Dropdown:', dropdown);
    log.info('   - display:', getComputedStyle(dropdown).display);
    log.info('   - visibility:', getComputedStyle(dropdown).visibility);
    log.info('   - opacity:', getComputedStyle(dropdown).opacity);
    log.info('   - z-index:', getComputedStyle(dropdown).zIndex);
    log.info('   - top:', getComputedStyle(dropdown).top);
    log.info('   - left:', getComputedStyle(dropdown).left);
    log.info('   - children:', dropdown?.children.length);
    
    log.info('4. Computed styles for hover:');
    const containerStyles = window.getComputedStyle(container);
    log.info('   - container hover transition:', containerStyles.transition);
    
    log.info('5. Test suggestions:');
    log.info('   - Run: testDropdownHover() to force show menu for 5s');
    log.info('   - Run: watchDropdownEvents() to monitor all events');
};

window.testDropdownHover = function() {
    const dropdown = document.getElementById('convertDropdown');
    if (!dropdown) {
        log.error('❌ Dropdown not found');
        return;
    }
    
    log.info('🧪 Force showing menu for 5 seconds...');
    dropdown.style.display = 'block';
    dropdown.style.opacity = '1';
    dropdown.style.visibility = 'visible';
    
    setTimeout(() => {
        log.info('✅ 5 seconds ended, restored to normal');
        dropdown.style.display = '';
        dropdown.style.opacity = '';
        dropdown.style.visibility = '';
    }, 5000);
};

window.watchDropdownEvents = function() {
    const container = document.getElementById('convertBtnContainer');
    const dropdown = document.getElementById('convertDropdown');
    
    if (!container || !dropdown) {
        log.error('❌ Elements not found');
        return;
    }
    
    log.info('👁️ Started monitoring events... (10s)');
    
    const events = ['mouseenter', 'mouseleave', 'mouseover', 'mouseout', 'mousemove'];
    const handlers = [];
    
    events.forEach(eventName => {
        const handler = (e) => {
            log.info(`[${eventName}] target:`, e.target.id || e.target.className, 
                       'display:', getComputedStyle(dropdown).display);
        };
        container.addEventListener(eventName, handler);
        dropdown.addEventListener(eventName, handler);
        handlers.push({ element: container, event: eventName, handler });
        handlers.push({ element: dropdown, event: eventName, handler });
    });
    
    setTimeout(() => {
        log.info('⏰ Monitoring ended, cleaned up event listeners');
        handlers.forEach(({ element, event, handler }) => {
            element.removeEventListener(event, handler);
        });
    }, 10000);
};

log.info('🔧 Debug tools loaded!');
log.info('💡 Run in console:');
log.info('   debugDropdown()       - View current state');
log.info('   testDropdownHover()   - Force show for 5s');
log.info('   watchDropdownEvents() - Monitor events for 10s');
