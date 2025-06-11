import { ChevronDown } from 'lucide-react';

import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from '@/components/ui/dropdown-menu';
import { SidebarMenu, SidebarMenuButton, SidebarMenuItem } from '@/components/ui/sidebar';

export function LoggedInUser() {
	return (
		<SidebarMenu>
			<SidebarMenuItem>
				<DropdownMenu>
					<DropdownMenuTrigger asChild>
						<SidebarMenuButton
							size="lg"
							className="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
						>
							<img
								src="https://ui-avatars.com/api/?name=admin"
								className="aspect-square size-8 items-center justify-center rounded-lg"
							/>
							<div className="flex flex-col gap-0.5 leading-none">
								<span className="font-medium">admin</span>
								<span className="">admin@yopmail.com</span>
							</div>
							<ChevronDown className="ml-auto" />
						</SidebarMenuButton>
					</DropdownMenuTrigger>
					<DropdownMenuContent className="w-(--radix-dropdown-menu-trigger-width)" align="start">
						<DropdownMenuItem>Account Setting</DropdownMenuItem>
						<DropdownMenuItem>Logout</DropdownMenuItem>
					</DropdownMenuContent>
				</DropdownMenu>
			</SidebarMenuItem>
		</SidebarMenu>
	);
}
