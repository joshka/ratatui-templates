let username = variable::get("username");
let full_name = variable::prompt("Full name", `${username}`);
variable::set("full-name", full_name);

let github_username = variable::prompt("GitHub username", "");
let project_name = variable::get("project-name");
let repo = variable::prompt("Git repository", `https://github.com/${github_username}/${project_name}`);
variable::set("github-repository", repo);