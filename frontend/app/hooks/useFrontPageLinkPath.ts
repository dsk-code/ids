export interface FrontPageLinkPath {
    homePath: HomePath,
    dashboardPath: DashboardPath,
    classesPath: ClassesPath,
    teachersPath: TeachersPath,
}

export type HomePath = "/";
export type DashboardPath = "/dashboard";
export type ClassesPath = "/dashboard/classes";
export type ClassPath = `/dashboard/classes/${string}`;
export type TeachersPath = "/dashboard/teachers";
export type TeacherPath = `/dashboard/teachers/${string}`;

export const useFrontPageLinkPath = () => {
    const frontPageLinkPath: FrontPageLinkPath = {
        homePath: "/",
        dashboardPath: "/dashboard",
        classesPath: "/dashboard/classes",
        teachersPath: "/dashboard/teachers",
    };

    return { frontPageLinkPath }
}