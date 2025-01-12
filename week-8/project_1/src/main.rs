fn main() {
    // Defining APS levels and roles using vectors
    let role  = vec![
        ("APS 1-2", vec!["Intern", "Paralegal", "Placement"]),
        ("APS 3-5", vec!["Administrator", "Research Assistant", "Junior Associate", "Classroom Teacher"]),
        ("APS 5-8", vec!["Senior Administrator", "PhD Candidate", "Associate", "Snr Teacher"]),
        ("EL1 8-10", vec!["Office Manager", "Post-Doc Researcher", "Senior Associate 1-2", "Leading Teacher"]),
        ("EL2 10-13", vec!["Director", "Senior Lecturer", "Senior Associate 3-4", "Deputy Principal"]),
        ("SES", vec!["CEO", "Dean", "Partner", "Principal"]),
    ];

    // Inputs
    let role = "Associate";
    let years_of_experience = 6;

    // Accessing APS levels using iteration
    let mut aps_level = "Role not found in the APS structure.".to_string();
    for &(rol, level) in &aps in &aps_levels {
        if aps.1.contains(&role) {
            aps_level = format!(
                "Role '{}' with {} years of experience is in {}.",
                role, years_of_experience, aps.0
            );
            break;
        }
    }

    // Output
    println!("{}", aps_level);
}
