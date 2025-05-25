use axum::{extract::Path, response::Html};

#[utoipa::path(
    get,
    path = "/webhook/{token}",
    summary = "Get information about the Keeper service",
    description = "Saves jwt token for the specified ID",
    responses(
        (status = 201, description = "HTML Page", body = [String])
    )
)]
#[axum::debug_handler]
pub async fn webhook(_token: Path<String>) -> Html<String> {
    Html(r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Sui Squad Login</title>
        <style>
            body {
                font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
                background-color: #f5f5f5;
                margin: 0;
                padding: 0;
                display: flex;
                justify-content: center;
                align-items: center;
                min-height: 100vh;
                color: #333;
            }
            .container {
                background-color: white;
                border-radius: 10px;
                box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
                padding: 30px;
                width: 90%;
                max-width: 500px;
                text-align: center;
            }
            h1 {
                color: #4a6cf7;
                margin-bottom: 20px;
            }
            .status-container {
                margin: 20px 0;
                padding: 15px;
                border-radius: 5px;
                background-color: #f9f9f9;
            }
            #status {
                font-size: 18px;
                font-weight: 500;
            }
            .spinner {
                border: 4px solid rgba(0, 0, 0, 0.1);
                width: 36px;
                height: 36px;
                border-radius: 50%;
                border-left-color: #4a6cf7;
                animation: spin 1s linear infinite;
                margin: 20px auto;
            }
            @keyframes spin {
                0% { transform: rotate(0deg); }
                100% { transform: rotate(360deg); }
            }
            .message {
                background-color: #f9f9f9;
                padding: 15px;
                border-radius: 5px;
                margin-top: 20px;
                font-size: 16px;
            }
            .success {
                color: #2ecc71;
                background-color: #d5f4e6;
                border: 1px solid #2ecc71;
            }
            .error {
                color: #e74c3c;
                background-color: #fdf2f2;
                border: 1px solid #e74c3c;
            }
            .warning {
                color: #f39c12;
                background-color: #fef9e7;
                border: 1px solid #f39c12;
            }
            .redirect-info {
                margin-top: 15px;
                padding: 10px;
                background-color: #e8f4fd;
                border-radius: 5px;
                color: #1e3a8a;
            }
            .countdown {
                font-weight: bold;
                color: #4a6cf7;
            }
        </style>
        <script>
            document.addEventListener('DOMContentLoaded', function() {
                // Parse the URL fragment (everything after #)
                const fragment = window.location.hash.substring(1);
                
                // Parse the fragment to get id_token and state
                const params = new URLSearchParams(fragment);
                const idToken = params.get('id_token');
                const state = params.get('state');
                
                let username = null;
                
                // Parse username from state if it exists
                if (state) {
                    try {
                        // Decode the state parameter (it might be URL encoded)
                        const decodedState = decodeURIComponent(state);
                        
                        // Try to parse as JSON first
                        try {
                            const stateObj = JSON.parse(decodedState);
                            username = stateObj.username || stateObj.user || stateObj.name;
                        } catch {
                            // If not JSON, try to extract username using regex or simple parsing
                            const usernameMatch = decodedState.match(/username[=:]([^&;,\s]+)/i);
                            if (usernameMatch) {
                                username = usernameMatch[1];
                            } else {
                                // If no explicit username field, use the entire state as username
                                username = decodedState;
                            }
                        }
                    } catch (error) {
                        console.error('Error parsing state:', error);
                    }
                }
                
                if (idToken) {
                    // Show processing state
                    document.getElementById('spinner').style.display = 'block';
                    document.getElementById('status').textContent = 'Processing login...';
                    
                    // Prepare request body
                    const requestBody = { token: idToken };
                    if (username) {
                        requestBody.username = username;
                    }
                    
                    // Send the token and username to the /keep endpoint
                    fetch('/keep', {
                        method: 'POST',
                        headers: {
                            'Content-Type': 'application/json',
                        },
                        body: JSON.stringify(requestBody),
                    })
                    .then(response => {
                        if (!response.ok) {
                            throw new Error(`Server responded with status: ${response.status}`);
                        }
                        return response.json();
                    })
                    .then(data => {
                        document.getElementById('spinner').style.display = 'none';
                        document.getElementById('status').textContent = 'Login successful!';
                        document.getElementById('status').className = 'success';
                        
                        const messageDiv = document.getElementById('message');
                        messageDiv.className = 'message success';
                        
                        let successMessage = '🎉 Welcome to Sui Squad!';
                        if (username) {
                            successMessage += `\n\nHello ${username}! Your account has been successfully linked.`;
                        } else {
                            successMessage += '\n\nYour account has been successfully linked.';
                        }
                        successMessage += '\n\nYou will be redirected to the Sui Squad Bot in a few seconds...';
                        
                        messageDiv.textContent = successMessage;
                        messageDiv.style.whiteSpace = 'pre-line';
                        
                        // Show redirect countdown
                        let countdown = 3;
                        const redirectDiv = document.getElementById('redirect-info');
                        redirectDiv.style.display = 'block';
                        
                        const countdownTimer = setInterval(() => {
                            document.getElementById('countdown').textContent = countdown;
                            countdown--;
                            
                            if (countdown < 0) {
                                clearInterval(countdownTimer);
                                // Redirect to Telegram bot
                                window.location.href = 'https://t.me/spiel_squard_test_bot';
                            }
                        }, 1000);
                        
                        // Initial countdown display
                        document.getElementById('countdown').textContent = countdown;
                    })
                    .catch(error => {
                        document.getElementById('spinner').style.display = 'none';
                        document.getElementById('status').textContent = 'Login failed';
                        document.getElementById('status').className = 'error';
                        
                        const messageDiv = document.getElementById('message');
                        messageDiv.className = 'message error';
                        messageDiv.textContent = `❌ Sorry, we couldn't complete your login.\n\nError: ${error.message}\n\nPlease try again or contact support if the problem persists.`;
                        messageDiv.style.whiteSpace = 'pre-line';
                    });
                } else {
                    document.getElementById('spinner').style.display = 'none';
                    document.getElementById('status').textContent = 'Invalid login link';
                    document.getElementById('status').className = 'warning';
                    
                    const messageDiv = document.getElementById('message');
                    messageDiv.className = 'message warning';
                    messageDiv.textContent = '⚠️ No login token found in the URL.\n\nThis usually means the login link is invalid or has expired.\n\nPlease try logging in again from the Sui Squad Bot.';
                    messageDiv.style.whiteSpace = 'pre-line';
                }
            });
        </script>
    </head>
    <body>
        <div class="container">
            <h1>Sui Squad Login Processing</h1>
            <div class="status-container">
                <div id="spinner" class="spinner"></div>
                <p id="status">Processing login...</p>
            </div>
            <div id="message" class="message"></div>
            <div id="redirect-info" class="redirect-info" style="display: none;">
                Redirecting to Sui Squad Bot in <span id="countdown" class="countdown">3</span> seconds...
            </div>
        </div>
    </body>
    </html>
    "#.to_string())
}
