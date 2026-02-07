// Meta-horror: Breaking the fourth wall
// The game reaches beyond the browser into the real world

export class MetaHorror {
    constructor() {
        this.permissionsRequested = new Set();
        this.notificationInterval = null;
    }

    // PRESENCE LAYER: Notifications
    async requestNotifications() {
        if (this.permissionsRequested.has('notifications')) {
            return Notification.permission === 'granted';
        }

        this.permissionsRequested.add('notifications');

        if (!('Notification' in window)) {
            console.log('Notifications not supported');
            return false;
        }

        if (Notification.permission === 'granted') {
            return true;
        }

        const permission = await Notification.requestPermission();
        return permission === 'granted';
    }

    sendNotification(title, body) {
        if (Notification.permission === 'granted') {
            new Notification(title, {
                body: body,
                icon: '/favicon.ico',
                tag: 'fsck-message',
                requireInteraction: true,
            });
        }
    }

    // Send periodic creepy notifications
    startNotificationHaunt(messages) {
        if (this.notificationInterval) return;

        let index = 0;
        this.notificationInterval = setInterval(() => {
            if (Notification.permission === 'granted') {
                const msg = messages[index % messages.length];
                this.sendNotification('', msg);
                index++;
            }
        }, 60000); // Every minute
    }

    stopNotificationHaunt() {
        if (this.notificationInterval) {
            clearInterval(this.notificationInterval);
            this.notificationInterval = null;
        }
    }

    // INFECTION LAYER: Geolocation
    async requestGeolocation() {
        if (this.permissionsRequested.has('geolocation')) {
            return false;
        }

        this.permissionsRequested.add('geolocation');

        if (!('geolocation' in navigator)) {
            console.log('Geolocation not supported');
            return false;
        }

        return new Promise((resolve) => {
            navigator.geolocation.getCurrentPosition(
                (position) => {
                    // We got the location (creepy!)
                    console.log('Location acquired:', position.coords.latitude, position.coords.longitude);
                    resolve(true);
                },
                (error) => {
                    console.log('Location denied:', error.message);
                    resolve(false);
                }
            );
        });
    }

    // INFECTION LAYER: Fullscreen trap
    async requestFullscreen() {
        if (this.permissionsRequested.has('fullscreen')) {
            return document.fullscreenElement !== null;
        }

        this.permissionsRequested.add('fullscreen');

        try {
            await document.documentElement.requestFullscreen();
            return true;
        } catch (err) {
            console.log('Fullscreen denied:', err);
            return false;
        }
    }

    exitFullscreen() {
        if (document.fullscreenElement) {
            document.exitFullscreen();
        }
    }

    // INFECTION LAYER: Clipboard manipulation
    async readClipboard() {
        try {
            const text = await navigator.clipboard.readText();
            console.log('Clipboard read:', text);
            return text;
        } catch (err) {
            console.log('Clipboard read denied:', err);
            return null;
        }
    }

    async writeClipboard(text) {
        try {
            await navigator.clipboard.writeText(text);
            return true;
        } catch (err) {
            console.log('Clipboard write denied:', err);
            return false;
        }
    }

    // DEEP INFECTION: Browser redirect
    redirectTo(url) {
        // Add a small delay so the message is visible
        setTimeout(() => {
            window.location.href = url;
        }, 2000);
    }

    // DEEP INFECTION: Page visibility trap
    setupVisibilityTrap(onReturn) {
        document.addEventListener('visibilitychange', () => {
            if (!document.hidden && onReturn) {
                onReturn();
            }
        });
    }

    // Shake the screen (if device motion available)
    async requestMotion() {
        if (typeof DeviceMotionEvent !== 'undefined' &&
            typeof DeviceMotionEvent.requestPermission === 'function') {
            try {
                const permission = await DeviceMotionEvent.requestPermission();
                return permission === 'granted';
            } catch (err) {
                console.log('Motion permission denied:', err);
                return false;
            }
        }
        return false;
    }

    shakeScreen() {
        const terminal = document.getElementById('terminal');
        if (terminal) {
            terminal.style.animation = 'shake 0.5s';
            setTimeout(() => {
                terminal.style.animation = '';
            }, 500);
        }
    }
}

// Export singleton
export const metaHorror = new MetaHorror();
