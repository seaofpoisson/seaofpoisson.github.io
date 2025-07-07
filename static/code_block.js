function toggle_code_block(block_id) {
    let block = document.getElementById(block_id);
    if (localStorage.getItem('show_code_blocks') === 'true') {
        localStorage.setItem('show_code_blocks', 'false');
        block.style.display = 'none';
    } else {
        localStorage.setItem('show_code_blocks', 'true');
        block.style.display = 'table-row';
    }
}

function show_or_hide_code_block(block_id) {
    let block = document.getElementById(block_id);
    if (localStorage.getItem('show_code_blocks') === 'true') {
        block.style.display = 'table-row';
    } else {
        block.style.display = 'none';
    }
}
