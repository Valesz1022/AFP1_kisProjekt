let login_elements: {
    nav_bar_reg_button: HTMLButtonElement
}

window.addEventListener('load', () => {
    login_elements = {
        nav_bar_reg_button:
            document.getElementById('nav_bar_reg_button') as HTMLButtonElement
    }

    login_elements.nav_bar_reg_button.addEventListener('click', () => {
        window.api.load_reg_page()
    })
})
