function show_page(old_id, new_id) {
    var old_page = document.getElementById(old_id);
    var new_page = document.getElementById(new_id);
    old_page.style.display = 'none';
    new_page.style.display = 'block';
    new_page.scrollIntoView();
}
