// @ts-check

const currentUrl = "/" + window.location.pathname.split('/')[1];
const element = document.querySelector(`nav li a[href="${currentUrl}"]`);
element?.classList.add('active');

