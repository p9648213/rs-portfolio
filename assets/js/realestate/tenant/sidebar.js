export function setupTenantSidebar() {
  const sidebar = document.getElementById("tenant-sidebar");
  const sidebarBtn = document.getElementById("tenant-sidebar-button");

  sidebarBtn.addEventListener("click", () => {
    sidebar.classList.toggle("close");
  });
}
