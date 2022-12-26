describe("create First Todo and find it", () => {
  it("successfully todo created", () => {
    cy.visit("/");
    cy.get("#new-todo").type("First Todo");
    cy.get("#new-todo").type("{enter}");
    cy.contains("First Todo")
  });
});
