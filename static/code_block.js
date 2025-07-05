function toggle_code_block(block_id) {
    let block = document.getElementById(block_id);
    if (block.style.display == 'none') {
        block.style.display = 'table-row';
    } else {
        block.style.display = 'none';
    }
}
