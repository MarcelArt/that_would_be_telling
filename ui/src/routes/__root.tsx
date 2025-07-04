import { AppSidebar } from '@/components/app-sidebar';
import { ThemeProvider } from '@/components/theme-provider';
import { Separator } from '@/components/ui/separator';
import { SidebarInset, SidebarProvider, SidebarTrigger } from '@/components/ui/sidebar';
import { Outlet, createRootRoute, useLocation } from '@tanstack/react-router';
import { TanStackRouterDevtools } from '@tanstack/react-router-devtools';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';

const queryClient = new QueryClient();

export const Route = createRootRoute({
	component: () => {
		const location = useLocation();
		const hideSidebarRoutes = ['/login'];
		const shouldHideSidebar = hideSidebarRoutes.includes(location.pathname);

		return (
			<QueryClientProvider client={queryClient}>
				<ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
					{!shouldHideSidebar ? (
						<SidebarProvider>
							<AppSidebar />
							<SidebarInset>
								<header className="flex h-16 shrink-0 items-center gap-2 border-b px-4">
									<SidebarTrigger className="-ml-1" />
									<Separator orientation="vertical" className="mr-2 data-[orientation=vertical]:h-4" />
									{/* <Breadcrumb>
            <BreadcrumbList>
              <BreadcrumbItem className="hidden md:block">
                <BreadcrumbLink href="#">
                  Building Your Application
                </BreadcrumbLink>
              </BreadcrumbItem>
              <BreadcrumbSeparator className="hidden md:block" />
              <BreadcrumbItem>
                <BreadcrumbPage>Data Fetching</BreadcrumbPage>
              </BreadcrumbItem>
            </BreadcrumbList>
          </Breadcrumb> */}
								</header>
								<Outlet />
								{/* <div className="flex flex-1 flex-col gap-4 p-4">
						<div className="grid auto-rows-min gap-4 md:grid-cols-3">
							<div className="bg-muted/50 aspect-video rounded-xl" />
							<div className="bg-muted/50 aspect-video rounded-xl" />
							<div className="bg-muted/50 aspect-video rounded-xl" />
						</div>
						<div className="bg-muted/50 min-h-[100vh] flex-1 rounded-xl md:min-h-min" />
					</div> */}
							</SidebarInset>
						</SidebarProvider>
					) : (
						<Outlet />
					)}
				</ThemeProvider>

				<TanStackRouterDevtools />
			</QueryClientProvider>
		);
	},
});
