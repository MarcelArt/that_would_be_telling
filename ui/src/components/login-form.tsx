import { cn } from '@/lib/utils';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Checkbox } from './ui/checkbox';
import { useState } from 'react';
import { useMutation } from '@tanstack/react-query';
import userApi from '@/api/user.api';
import { useNavigate } from '@tanstack/react-router';

export function LoginForm({ className, ...props }: React.ComponentProps<'div'>) {
	const [username, setUsername] = useState('');
	const [password, setPassword] = useState('');
	const [isRemember, setIsRemember] = useState(false);

	const navigate = useNavigate();

	const { mutate } = useMutation({
		mutationFn: async () => {
			return userApi.login({
				is_remember: isRemember,
				password,
				username,
			});
		},
		onSuccess: (data) => {
			localStorage.setItem('accessToken', data.data.access_token);
			localStorage.setItem('refreshToken', data.data.refresh_token);
			navigate({ to: '/' });
		},
	});

	return (
		<div className={cn('flex flex-col gap-6', className)} {...props}>
			<Card>
				<CardHeader>
					<CardTitle>Login to your account</CardTitle>
					<CardDescription>Enter your username below to login to your account</CardDescription>
				</CardHeader>
				<CardContent>
					<div className="flex flex-col gap-6">
						<div className="grid gap-3">
							<Label htmlFor="username">Username</Label>
							<Input
								id="username"
								type="text"
								placeholder="admin"
								required
								value={username}
								onChange={(e) => setUsername(e.target.value)}
							/>
						</div>
						<div className="grid gap-3">
							<div className="flex items-center">
								<Label htmlFor="password">Password</Label>
								<a href="#" className="ml-auto inline-block text-sm underline-offset-4 hover:underline">
									Forgot your password?
								</a>
							</div>
							<Input id="password" type="password" required value={password} onChange={(e) => setPassword(e.target.value)} />
						</div>
						<div className="flex items-center gap-3">
							<Checkbox id="is_remember_me" checked={isRemember} onCheckedChange={(e) => setIsRemember(e.valueOf() as boolean)} />
							<Label htmlFor="is_remember_me">Remember Me</Label>
						</div>
						<div className="flex flex-col gap-3">
							<Button className="w-full" onClick={() => mutate()}>
								Login
							</Button>
						</div>
					</div>
				</CardContent>
			</Card>
		</div>
	);
}
