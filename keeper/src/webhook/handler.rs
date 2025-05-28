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
                
                let userId = null;
                let botId = null;
                let network = null;
                let publicKey = null;
                let maxEpoch = null;
                let randomness = null;
                
                // Parse userId from state if it exists
                if (state) {
                    try {
                        // Decode the state parameter (it might be URL encoded)
                        const decodedState = decodeURIComponent(state);
                        
                        // Try to parse as JSON first
                        try {
                            const stateObj = JSON.parse(decodedState);
                            userId = stateObj.user_id || stateObj.user || stateObj.name;
                            botId = stateObj.bot_id || stateObj.botId || stateObj.bot_id;
                            network = stateObj.network;
                            publicKey = stateObj.public_key || stateObj.publicKey;
                            maxEpoch = stateObj.max_epoch || stateObj.maxEpoch;
                            randomness = stateObj.randomness;
                        } catch {
                            // If not JSON, try to extract userId using regex or simple parsing
                            const userIdMatch = decodedState.match(/user_id[=:]([^&;,\s]+)/i);
                            if (userIdMatch) {
                                userId = userIdMatch[1];
                            } else {
                                // If no explicit userId field, use the entire state as userId
                                userId = decodedState;
                            }
                            
                            // Try to extract bot_id using regex
                            const botIdMatch = decodedState.match(/bot_id[=:]([^&;,\s]+)/i);
                            if (botIdMatch) {
                                botId = botIdMatch[1];
                            }
                            
                            // Try to extract other parameters using regex
                            const networkMatch = decodedState.match(/network[=:]([^&;,\s]+)/i);
                            if (networkMatch) {
                                network = networkMatch[1];
                            }
                            
                            const publicKeyMatch = decodedState.match(/public_key[=:]([^&;,\s]+)/i);
                            if (publicKeyMatch) {
                                publicKey = publicKeyMatch[1];
                            }
                            
                            const maxEpochMatch = decodedState.match(/max_epoch[=:]([^&;,\s]+)/i);
                            if (maxEpochMatch) {
                                maxEpoch = parseInt(maxEpochMatch[1]);
                            }
                            
                            const randomnessMatch = decodedState.match(/randomness[=:]([^&;,\s]+)/i);
                            if (randomnessMatch) {
                                randomness = randomnessMatch[1];
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
                    if (userId) {
                        requestBody.user_id = userId;
                    }
                    if (botId) {
                        requestBody.bot_id = botId;
                    }
                    if (network) {
                        requestBody.network = network;
                    }
                    if (publicKey) {
                        requestBody.public_key = publicKey;
                    }
                    if (maxEpoch !== null) {
                        requestBody.max_epoch = maxEpoch;
                    }
                    if (randomness) {
                        requestBody.randomness = randomness;
                    }
                    
                    // Send the token and userId to the /keep endpoint
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
                        if (userId) {
                            successMessage += `\n\nHello ${userId}! Your account has been successfully linked.`;
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
