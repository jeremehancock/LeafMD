// Shows a Copy button next to the install command and copies it when
// clicked. Without scripts the button stays hidden, and the command is still
// there as plain text to select and copy by hand.
document.querySelectorAll("button.copy").forEach((button) => {
  const source = document.getElementById(button.dataset.copy);
  if (!source || !navigator.clipboard) return;
  button.hidden = false;
  button.addEventListener("click", async () => {
    try {
      await navigator.clipboard.writeText(source.textContent.trim());
      button.textContent = "Copied";
    } catch {
      button.textContent = "Select and copy";
    }
    setTimeout(() => { button.textContent = "Copy"; }, 2000);
  });
});
