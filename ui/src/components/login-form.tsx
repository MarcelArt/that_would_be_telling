import { cn } from '@/lib/utils';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Checkbox } from './ui/checkbox';

export function LoginForm({ className, ...props }: React.ComponentProps<'div'>) {
	return (
		<div className={cn('flex flex-col gap-6', className)} {...props}>
			<Card>
				<CardHeader>
					<CardTitle>Login to your account</CardTitle>
					<CardDescription>Enter your username below to login to your account</CardDescription>
				</CardHeader>
				<CardContent>
					<form>
						<div className="flex flex-col gap-6">
							<div className="grid gap-3">
								<Label htmlFor="username">Username</Label>
								<Input id="username" type="text" placeholder="admin" required />
							</div>
							<div className="grid gap-3">
								<div className="flex items-center">
									<Label htmlFor="password">Password</Label>
									<a href="#" className="ml-auto inline-block text-sm underline-offset-4 hover:underline">
										Forgot your password?
									</a>
								</div>
								<Input id="password" type="password" required />
							</div>
							<div className="flex items-center gap-3">
								<Checkbox id="is_remember_me" />
								<Label htmlFor="is_remember_me">Remember Me</Label>
							</div>
							<div className="flex flex-col gap-3">
								<Button type="submit" className="w-full">
									Login
								</Button>
							</div>
						</div>
					</form>
				</CardContent>
			</Card>
		</div>
	);
}
