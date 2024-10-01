// Define the URL for the API endpoint where you want to send the form data.
const loginUrl = 'https://your-api-endpoint.com/login'; // Replace with your actual API endpoint

// Get references to the form and result display elements
const loginForm = document.querySelector('form') as HTMLFormElement;
const loginButton = document.getElementById('login_button') as HTMLButtonElement;

// Async function to send the login data to the server
async function sendLoginData(username: string, password: string): Promise<void> {
    try {
        // Create the login payload
        const loginData = {
            username: username,
            password: password
        };

        // Send a POST request to the server
        const response = await fetch(loginUrl, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(loginData)
        });

        // Handle the server response
        if (response.ok) {
            const responseData = await response.json();
            console.log('Login successful:', responseData);
            // You can add further actions here, like redirecting or showing a success message
        } else {
            console.error('Login failed:', response.status);
            // Handle login failure, e.g., show an error message to the user
        }
    } catch (error) {
        console.error('An error occurred:', error);
        // Handle general errors, such as network problems
    }
}

// Event listener for form submission
loginForm.addEventListener('submit', (event) => {
    event.preventDefault(); // Prevent the default form submission (page reload)

    // Get the username and password values from the form
    const username = (document.getElementById('login_username') as HTMLInputElement).value;
    const password = (document.getElementById('login_password') as HTMLInputElement).value;

    // Call the async function to send the login data
    sendLoginData(username, password);
});
