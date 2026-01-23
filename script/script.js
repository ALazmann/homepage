document.addEventListener('DOMContentLoaded', () => {
    
    // 1. Typing Effect
    const typingContainer = document.querySelector('.typing-text');
    if (typingContainer) {
        const text = "Human-Computer Interaction Student.";
        let index = 0;

        function type() {
            if (index < text.length) {
                typingContainer.innerHTML += text.charAt(index);
                index++;
                setTimeout(type, 70);
            }
        }
        type();
    }

    // 2. Scroll Reveal Animation
    const revealElements = document.querySelectorAll('.reveal');
    
    const revealOnScroll = () => {
        revealElements.forEach(el => {
            const elementTop = el.getBoundingClientRect().top;
            const windowHeight = window.innerHeight;
            
            if (elementTop < windowHeight - 100) {
                el.classList.add('active');
            }
        });
    };

    // Run once on load and then on scroll
    window.addEventListener('scroll', revealOnScroll);
    revealOnScroll();
});
