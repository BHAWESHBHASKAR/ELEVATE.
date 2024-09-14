document.addEventListener('DOMContentLoaded', () => {
    const navOpenBtn = document.querySelector('.nav-open-btn');
    const navCloseBtn = document.querySelector('.navbar .nav-close-btn');
    const navbar = document.querySelector('.navbar');

    const navOpenBtn2 = document.querySelector('.nav-open-btn2');
    const navCloseBtn2 = document.querySelector('.navbar2 .nav-close-btn2');
    const navbar2 = document.querySelector('.navbar2');

    if (navOpenBtn) {
        navOpenBtn.addEventListener('click', () => {
            navbar.classList.add('open');
        });
    }

    if (navCloseBtn) {
        navCloseBtn.addEventListener('click', () => {
            navbar.classList.remove('open');
        });
    }

    if (navOpenBtn2) {
        navOpenBtn2.addEventListener('click', () => {
            navbar2.classList.add('open');
        });
    }

    if (navCloseBtn2) {
        navCloseBtn2.addEventListener('click', () => {
            navbar2.classList.remove('open');
        });
    }
});
