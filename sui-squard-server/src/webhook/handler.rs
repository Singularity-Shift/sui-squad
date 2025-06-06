use axum::{extract::Path, response::Html};
use std::env;

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
    let enoki_api_key = env::var("ENOKI_API_KEY").unwrap_or_else(|_| "".to_string());
    let redirect_back = env::var("REDIRECT_BACK").unwrap_or_else(|_| "".to_string());

    let html_content = format!(
        r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Sui Squad Login</title>
        <style>
            body {{
                font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
                background-color: #f5f5f5;
                margin: 0;
                padding: 0;
                display: flex;
                justify-content: center;
                align-items: center;
                min-height: 100vh;
                color: #333;
            }}
            .container {{
                background-color: white;
                border-radius: 10px;
                box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
                padding: 30px;
                width: 90%;
                max-width: 500px;
                text-align: center;
            }}
            h1 {{
                color: #4a6cf7;
                margin-bottom: 20px;
            }}
            .status-container {{
                margin: 20px 0;
                padding: 15px;
                border-radius: 5px;
                background-color: #f9f9f9;
            }}
            #status {{
                font-size: 18px;
                font-weight: 500;
            }}
            .spinner {{
                border: 4px solid rgba(0, 0, 0, 0.1);
                width: 36px;
                height: 36px;
                border-radius: 50%;
                border-left-color: #4a6cf7;
                animation: spin 1s linear infinite;
                margin: 20px auto;
            }}
            @keyframes spin {{
                0% {{ transform: rotate(0deg); }}
                100% {{ transform: rotate(360deg); }}
            }}
            .message {{
                background-color: #f9f9f9;
                padding: 15px;
                border-radius: 5px;
                margin-top: 20px;
                font-size: 16px;
            }}
            .success {{
                color: #2ecc71;
                background-color: #d5f4e6;
                border: 1px solid #2ecc71;
            }}
            .error {{
                color: #e74c3c;
                background-color: #fdf2f2;
                border: 1px solid #e74c3c;
            }}
            .warning {{
                color: #f39c12;
                background-color: #fef9e7;
                border: 1px solid #f39c12;
            }}
            .address-container {{
                margin-top: 20px;
                padding: 15px;
                background-color: #f0f8ff;
                border-radius: 8px;
                border: 1px solid #4a6cf7;
            }}
            .address-text {{
                font-family: monospace;
                font-size: 14px;
                background-color: #f9f9f9;
                padding: 10px;
                border-radius: 5px;
                word-break: break-all;
                margin: 10px 0;
                border: 1px solid #ddd;
                position: relative;
            }}
            .copy-button {{
                background-color: #6c757d;
                color: white;
                border: none;
                padding: 8px 12px;
                border-radius: 4px;
                font-size: 12px;
                cursor: pointer;
                margin-left: 10px;
                transition: all 0.3s;
            }}
            .copy-button:hover {{
                background-color: #5a6268;
            }}
            .copy-button.copied {{
                background-color: #28a745;
            }}
            .address-row {{
                display: flex;
                align-items: center;
                justify-content: space-between;
                flex-wrap: wrap;
            }}
            .fund-button {{
                background-color: #4a6cf7;
                color: white;
                border: none;
                padding: 12px 24px;
                border-radius: 5px;
                font-size: 16px;
                cursor: pointer;
                margin-top: 15px;
                transition: background-color 0.3s;
            }}
            .fund-button:hover {{
                background-color: #3b5bdb;
            }}
            .fund-button:disabled {{
                background-color: #ccc;
                cursor: not-allowed;
            }}
        </style>
        <script>
            document.addEventListener('DOMContentLoaded', function() {{
                // Parse the URL fragment (everything after #)
                const fragment = window.location.hash.substring(1);
                
                // Parse the fragment to get id_token and state
                const params = new URLSearchParams(fragment);
                const idToken = params.get('id_token');
                const state = params.get('state');
                
                let publicKey = null;
                let maxEpoch = null;
                let telegramId = null;
                let randomness = null;

                // Parse userId from state if it exists
                if (state) {{
                    try {{
                        // Decode the state parameter (it might be URL encoded)
                        const decodedState = decodeURIComponent(state);
                        
                        // Try to parse as JSON first
                        try {{
                            const stateObj = JSON.parse(decodedState);

                            publicKey = stateObj.public_key || stateObj.publicKey;
                            maxEpoch = stateObj.max_epoch || stateObj.maxEpoch;
                            telegramId = stateObj.telegram_id || stateObj.telegramId;
                            randomness = stateObj.randomness;
                        }} catch {{
                            // If not JSON, try to extract userId using regex or simple parsing
                            const publicKeyMatch = decodedState.match(/public_key[=:]([^&;,\s]+)/i);
                            const maxEpochMatch = decodedState.match(/max_epoch[=:]([^&;,\s]+)/i);
                            const telegramIdMatch = decodedState.match(/telegram_id[=:]([^&;,\s]+)/i);
                            const randomnessMatch = decodedState.match(/randomness[=:]([^&;,\s]+)/i);

                            if (publicKeyMatch) {{
                                publicKey = publicKeyMatch[1];
                            }}

                            if (maxEpochMatch) {{
                                maxEpoch = maxEpochMatch[1];
                            }}

                            if (telegramIdMatch) {{
                                telegramId = telegramIdMatch[1];
                            }}

                            if (randomnessMatch) {{
                                randomness = randomnessMatch[1];
                            }}
                        }}
                    }} catch (error) {{
                        console.error('Error parsing state:', error);
                    }}
                }}
                
                if (idToken) {{
                    // Show processing state
                    document.getElementById('spinner').style.display = 'block';
                    document.getElementById('status').textContent = 'Getting your wallet address...';

                    // Call Enoki API to get address
                    fetch('https://api.enoki.mystenlabs.com/v1/zklogin', {{
                        method: 'GET',
                        headers: {{
                            'zklogin-jwt': idToken,
                            'Authorization': 'Bearer {enoki_api_key}',
                        }},
                    }})
                    .then(response => {{
                        if (!response.ok) {{
                            throw new Error(`Enoki API responded with status: ${{response.status}}`);
                        }}
                        return response.json();
                    }})
                    .then(response => {{
                        document.getElementById('spinner').style.display = 'none';
                        document.getElementById('status').textContent = 'Address Retrieved Successfully!';
                        document.getElementById('status').className = 'success';
                        
                        const messageDiv = document.getElementById('message');
                        messageDiv.className = 'message success';
                        messageDiv.innerHTML = `
                            <h3>💰 Fund Your Sui Squad Account</h3>
                            <p>Please transfer SUI tokens to the following address:</p>
                            <div class="address-container">
                                <div class="address-row">
                                    <div class="address-text" id="wallet-address">${{response.data.address}}</div>
                                    <button class="copy-button" onclick="copyAddress()" id="copy-btn">
                                        📋 Copy
                                    </button>
                                </div>
                                <p><small>Copy the address above and transfer SUI tokens to fund your account.</small></p>
                                <button class="fund-button" onclick="fundAccount('${{publicKey}}', '${{telegramId}}', '${{maxEpoch}}', '${{randomness}}', '${{idToken}}')">
                                    ✅ I've Sent the Transfer - Complete Funding
                                </button>
                            </div>
                        `;
                    }})
                    .catch(error => {{
                        document.getElementById('spinner').style.display = 'none';
                        document.getElementById('status').textContent = 'Failed to get address';
                        document.getElementById('status').className = 'error';
                        
                        const messageDiv = document.getElementById('message');
                        messageDiv.className = 'message error';
                        messageDiv.textContent = `❌ Sorry, we couldn't get your wallet address.\n\nError: ${{error.message}}\n\nPlease try again or contact support if the problem persists.`;
                        messageDiv.style.whiteSpace = 'pre-line';
                    }});
                }} else {{
                    document.getElementById('spinner').style.display = 'none';
                    document.getElementById('status').textContent = 'Invalid login link';
                    document.getElementById('status').className = 'warning';
                    
                    const messageDiv = document.getElementById('message');
                    messageDiv.className = 'message warning';
                    messageDiv.textContent = '⚠️ No login token found in the URL.\n\nThis usually means the login link is invalid or has expired.\n\nPlease try logging in again from the Sui Squad Bot.';
                    messageDiv.style.whiteSpace = 'pre-line';
                }}
            }});

            function copyAddress() {{
                const addressElement = document.getElementById('wallet-address');
                const copyButton = document.getElementById('copy-btn');
                const address = addressElement.textContent;

                navigator.clipboard.writeText(address).then(() => {{
                    // Visual feedback
                    copyButton.textContent = '✅ Copied!';
                    copyButton.classList.add('copied');
                    
                    // Reset button after 2 seconds
                    setTimeout(() => {{
                        copyButton.textContent = '📋 Copy';
                        copyButton.classList.remove('copied');
                    }}, 2000);
                }}).catch(err => {{
                    console.error('Failed to copy: ', err);
                    // Fallback for older browsers
                    try {{
                        const textArea = document.createElement('textarea');
                        textArea.value = address;
                        document.body.appendChild(textArea);
                        textArea.select();
                        document.execCommand('copy');
                        document.body.removeChild(textArea);
                        
                        copyButton.textContent = '✅ Copied!';
                        copyButton.classList.add('copied');
                        setTimeout(() => {{
                            copyButton.textContent = '📋 Copy';
                            copyButton.classList.remove('copied');
                        }}, 2000);
                    }} catch (fallbackErr) {{
                        copyButton.textContent = '❌ Failed';
                        setTimeout(() => {{
                            copyButton.textContent = '📋 Copy';
                        }}, 2000);
                    }}
                }});
            }}

            function fundAccount(publicKey, telegramId, maxEpoch, randomness, token) {{
                const button = event.target;
                button.disabled = true;
                button.textContent = 'Processing...';

                const requestBody = {{
                    telegram_id: telegramId,
                    public_key: publicKey,
                    max_epoch: parseInt(maxEpoch),
                    randomness: randomness,
                }};

                let auth = `Bearer ${{token}}`;
                
                // Call our /fund endpoint
                fetch('/fund', {{
                    method: 'POST',
                    headers: {{
                        'Content-Type': 'application/json',
                        'Authorization': auth,
                    }},
                    body: JSON.stringify(requestBody),
                }})
                .then(response => {{
                    if (!response.ok) {{
                        throw new Error(`Server responded with status: ${{response.status}}`);
                    }}
                    return response;
                }})
                .then(data => {{
                    button.textContent = '✅ Funding Complete!';
                    button.style.backgroundColor = '#2ecc71';
                    
                    const messageDiv = document.getElementById('message');
                    messageDiv.innerHTML += `
                        <div class="message success" style="margin-top: 15px;">
                            <h4>🎉 Funding Successful!</h4>
                            <p>Your Sui Squad account has been funded successfully.</p>
                            <div class="redirect-info" style="margin-top: 15px; padding: 10px; background-color: #e8f4fd; border-radius: 5px; color: #1e3a8a;">
                                Redirecting to Sui Squad Bot in <span id="countdown" class="countdown" style="font-weight: bold; color: #4a6cf7;">5</span> seconds...
                            </div>
                        </div>
                    `;
                    
                    // Start countdown and redirect
                    let countdown = 5;
                    const countdownElement = document.getElementById('countdown');
                    
                    const countdownTimer = setInterval(() => {{
                        countdown--;
                        countdownElement.textContent = countdown;
                        
                        if (countdown <= 0) {{
                            clearInterval(countdownTimer);
                            // Redirect to Telegram bot
                            window.location.href = '{redirect_back}';
                        }}
                    }}, 1000);
                }})
                .catch(error => {{
                    button.disabled = false;
                    button.textContent = '❌ Funding Failed - Try Again';
                    button.style.backgroundColor = '#e74c3c';
                    
                    const messageDiv = document.getElementById('message');
                    messageDiv.innerHTML += `
                        <div class="message error" style="margin-top: 15px;">
                            <h4>❌ Funding Failed</h4>
                            <p>Error: ${{error.message}}</p>
                            <p>Please try again or contact support if the problem persists.</p>
                        </div>
                    `;
                }});
            }}
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
        </div>
    </body>
    </html>
    "#,
        enoki_api_key = enoki_api_key,
        redirect_back = redirect_back
    );

    Html(html_content)
}
