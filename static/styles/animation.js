document.addEventListener("DOMContentLoaded", function() {
    const currentUrl = window.location.pathname;
    const element = document.querySelector(`nav li a[href="${currentUrl}"]`);
    element?.classList.add('active');
});

