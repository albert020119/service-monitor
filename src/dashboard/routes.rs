use axum::{Json, Extension, response::Html};
use crate::state::AppState;

pub async fn index() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Service Health Monitor</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            padding: 20px;
        }
        
        .container {
            max-width: 1200px;
            margin: 0 auto;
        }
        
        header {
            text-align: center;
            color: white;
            margin-bottom: 40px;
        }
        
        h1 {
            font-size: 3em;
            margin-bottom: 10px;
            text-shadow: 2px 2px 4px rgba(0,0,0,0.2);
        }
        
        .subtitle {
            font-size: 1.2em;
            opacity: 0.9;
        }
        
        .stats {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
            margin-bottom: 30px;
        }
        
        .stat-card {
            background: white;
            padding: 20px;
            border-radius: 12px;
            box-shadow: 0 4px 6px rgba(0,0,0,0.1);
            text-align: center;
        }
        
        .stat-value {
            font-size: 2.5em;
            font-weight: bold;
            margin-bottom: 5px;
        }
        
        .stat-value.up {
            color: #10b981;
        }
        
        .stat-value.down {
            color: #ef4444;
        }
        
        .stat-label {
            color: #6b7280;
            font-size: 0.9em;
            text-transform: uppercase;
            letter-spacing: 1px;
        }
        
        .services-grid {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
            gap: 20px;
        }
        
        .service-card {
            background: white;
            border-radius: 12px;
            padding: 24px;
            box-shadow: 0 4px 6px rgba(0,0,0,0.1);
            transition: transform 0.2s, box-shadow 0.2s;
        }
        
        .service-card:hover {
            transform: translateY(-4px);
            box-shadow: 0 8px 12px rgba(0,0,0,0.15);
        }
        
        .service-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 16px;
        }
        
        .service-name {
            font-size: 1.4em;
            font-weight: 600;
            color: #1f2937;
        }
        
        .status-badge {
            padding: 6px 16px;
            border-radius: 20px;
            font-size: 0.85em;
            font-weight: 600;
            text-transform: uppercase;
        }
        
        .status-badge.up {
            background: #d1fae5;
            color: #065f46;
        }
        
        .status-badge.down {
            background: #fee2e2;
            color: #991b1b;
        }
        
        .status-badge.unknown {
            background: #e5e7eb;
            color: #4b5563;
        }
        
        .service-url {
            color: #6b7280;
            font-size: 0.9em;
            margin-bottom: 12px;
            word-break: break-all;
        }
        
        .service-details {
            display: grid;
            grid-template-columns: repeat(2, 1fr);
            gap: 12px;
            margin-top: 16px;
            padding-top: 16px;
            border-top: 1px solid #e5e7eb;
        }
        
        .detail-item {
            display: flex;
            flex-direction: column;
        }
        
        .detail-label {
            color: #6b7280;
            font-size: 0.8em;
            margin-bottom: 4px;
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }
        
        .detail-value {
            color: #1f2937;
            font-size: 1.1em;
            font-weight: 600;
        }
        
        .message {
            margin-top: 12px;
            padding: 8px 12px;
            background: #f3f4f6;
            border-radius: 6px;
            font-size: 0.85em;
            color: #4b5563;
        }
        
        .loading {
            text-align: center;
            color: white;
            font-size: 1.2em;
            padding: 40px;
        }
        
        .last-updated {
            text-align: center;
            color: white;
            margin-top: 30px;
            opacity: 0.8;
        }
        
        .check-type {
            display: inline-block;
            padding: 4px 12px;
            background: #dbeafe;
            color: #1e40af;
            border-radius: 12px;
            font-size: 0.8em;
            font-weight: 600;
            margin-bottom: 8px;
        }
        
        .next-check {
            margin-top: 12px;
            padding: 8px 0;
        }
        
        .next-check-label {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 6px;
            font-size: 0.85em;
            color: #6b7280;
        }
        
        .next-check-time {
            font-weight: 600;
            color: #4b5563;
        }
        
        .progress-bar-container {
            width: 100%;
            height: 6px;
            background: #e5e7eb;
            border-radius: 3px;
            overflow: hidden;
        }
        
        .progress-bar {
            height: 100%;
            background: linear-gradient(90deg, #667eea 0%, #764ba2 100%);
            border-radius: 3px;
            transition: width 0.3s ease;
        }
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>Service Health Monitor</h1>
            <p class="subtitle">Real-time monitoring dashboard</p>
        </header>
        
        <div class="stats" id="stats">
            <div class="stat-card">
                <div class="stat-value up" id="upCount">-</div>
                <div class="stat-label">Services Up</div>
            </div>
            <div class="stat-card">
                <div class="stat-value down" id="downCount">-</div>
                <div class="stat-label">Services Down</div>
            </div>
            <div class="stat-card">
                <div class="stat-value" id="avgUptime">-</div>
                <div class="stat-label">Avg Uptime</div>
            </div>
            <div class="stat-card">
                <div class="stat-value" id="avgResponse">-</div>
                <div class="stat-label">Avg Response</div>
            </div>
        </div>
        
        <div class="services-grid" id="services">
            <div class="loading">Loading services...</div>
        </div>
        
        <div class="last-updated" id="lastUpdated"></div>
    </div>
    
    <script>
        let currentServices = [];
        
        async function fetchStatus() {
            try {
                const response = await fetch('/api/status');
                const data = await response.json();
                
                currentServices = data.services;
                updateStats(data.services);
                updateServices(data.services);
                
                document.getElementById('lastUpdated').textContent = 
                    'Last updated: ' + new Date().toLocaleTimeString();
            } catch (error) {
                console.error('Error fetching status:', error);
            }
        }
        
        function updateProgressBars() {
            if (currentServices.length === 0) return;
            
            currentServices.forEach((service, index) => {
                const lastCheck = new Date(service.last_check);
                const now = new Date();
                const elapsed = Math.floor((now - lastCheck) / 1000);
                const interval = service.interval_seconds;
                const remaining = Math.max(0, interval - elapsed);
                const progress = Math.min(100, (elapsed / interval) * 100);
                
                const timeElement = document.querySelector(`[data-next-check="${index}"]`);
                const progressElement = document.querySelector(`[data-progress="${index}"]`);
                
                if (timeElement) {
                    if (remaining > 0) {
                        timeElement.textContent = formatSeconds(remaining);
                    } else {
                        timeElement.textContent = 'checking...';
                    }
                }
                
                if (progressElement) {
                    progressElement.style.width = progress + '%';
                }
            });
        }
        
        function formatSeconds(seconds) {
            if (seconds < 60) {
                return seconds + 's';
            } else if (seconds < 3600) {
                const mins = Math.floor(seconds / 60);
                const secs = seconds % 60;
                return mins + 'm ' + secs + 's';
            } else {
                const hours = Math.floor(seconds / 3600);
                const mins = Math.floor((seconds % 3600) / 60);
                return hours + 'h ' + mins + 'm';
            }
        }
        
        function updateStats(services) {
            const upCount = services.filter(s => s.status === 'Up').length;
            const downCount = services.filter(s => s.status === 'Down').length;
            const avgUptime = services.length > 0 
                ? (services.reduce((sum, s) => sum + s.uptime_percentage, 0) / services.length).toFixed(1)
                : 0;
            const responseTimes = services
                .filter(s => s.response_time_ms !== null)
                .map(s => s.response_time_ms);
            const avgResponse = responseTimes.length > 0
                ? Math.round(responseTimes.reduce((sum, t) => sum + t, 0) / responseTimes.length)
                : 0;
            
            document.getElementById('upCount').textContent = upCount;
            document.getElementById('downCount').textContent = downCount;
            document.getElementById('avgUptime').textContent = avgUptime + '%';
            document.getElementById('avgResponse').textContent = avgResponse + 'ms';
        }
        
        function updateServices(services) {
            const container = document.getElementById('services');
            
            if (services.length === 0) {
                container.innerHTML = '<div class="loading">No services configured</div>';
                return;
            }
            
            container.innerHTML = services.map((service, index) => `
                <div class="service-card" data-service-index="${index}">
                    <div class="service-header">
                        <div class="service-name">${escapeHtml(service.name)}</div>
                        <div class="status-badge ${service.status.toLowerCase()}">
                            ${service.status}
                        </div>
                    </div>
                    <div class="check-type">${service.check_type}</div>
                    <div class="service-url">${escapeHtml(service.url)}</div>
                    <div class="service-details">
                        <div class="detail-item">
                            <div class="detail-label">Uptime</div>
                            <div class="detail-value">${service.uptime_percentage.toFixed(1)}%</div>
                        </div>
                        <div class="detail-item">
                            <div class="detail-label">Response</div>
                            <div class="detail-value">
                                ${service.response_time_ms !== null ? service.response_time_ms + 'ms' : 'N/A'}
                            </div>
                        </div>
                        <div class="detail-item">
                            <div class="detail-label">Checks</div>
                            <div class="detail-value">${service.total_checks}</div>
                        </div>
                        <div class="detail-item">
                            <div class="detail-label">Last Check</div>
                            <div class="detail-value">
                                ${formatTime(service.last_check)}
                            </div>
                        </div>
                    </div>
                    ${service.message ? `<div class="message">${escapeHtml(service.message)}</div>` : ''}
                    <div class="next-check">
                        <div class="next-check-label">
                            <span>Next check in</span>
                            <span class="next-check-time" data-next-check="${index}">calculating...</span>
                        </div>
                        <div class="progress-bar-container">
                            <div class="progress-bar" data-progress="${index}" style="width: 0%"></div>
                        </div>
                    </div>
                </div>
            `).join('');
            
            updateProgressBars();
        }
        
        function formatTime(timestamp) {
            const date = new Date(timestamp);
            const now = new Date();
            const diff = Math.floor((now - date) / 1000);
            
            if (diff < 60) return diff + 's ago';
            if (diff < 3600) return Math.floor(diff / 60) + 'm ago';
            if (diff < 86400) return Math.floor(diff / 3600) + 'h ago';
            return date.toLocaleDateString();
        }
        
        function escapeHtml(text) {
            const div = document.createElement('div');
            div.textContent = text;
            return div.innerHTML;
        }
        
        fetchStatus();
        setInterval(fetchStatus, 5000);
        setInterval(updateProgressBars, 1000);
    </script>
</body>
</html>
    "#)
}

pub async fn status(Extension(state): Extension<AppState>) -> Json<serde_json::Value> {
    let services = state.get_all_services().await;
    Json(serde_json::json!({ "services": services }))
}
