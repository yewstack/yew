describe("check if the links are attached", () => {
  it("successfully links are attached", () => {
    cy.visit("/");
    cy.get("a[href='https://github.com/DenisKolodin/']")
    cy.get("a[href='http://todomvc.com/']")
  });
});
